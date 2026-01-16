use std::{sync::Arc, time::Duration};

use crate::{
    account::{AccountDecoder, AccountPipe, AccountPipes, AccountProcessorInputType},
    block_meta::{BlockMetaPipe, BlockMetaPipes},
    datasource::{Datasource, DatasourceId},
    dedup::UpdateDeduper,
    error::SiliconResult,
    filter::Filter,
    instruction::{
        InstructionDecoder, InstructionPipe, InstructionPipes, InstructionProcessorInputType,
    },
    processor::Processor,
    update::{BlockMetaUpdate, Update, UpdateKey},
};

pub struct Pipeline {
    datasources: Vec<Arc<dyn Datasource>>,
    account_pipes: Vec<Box<dyn AccountPipes>>,
    block_meta_pipes: Vec<Box<dyn BlockMetaPipes>>,
    instruction_pipes: Vec<Box<dyn InstructionPipes>>,
    shutdown_pair: Option<(mea::shutdown::ShutdownSend, mea::shutdown::ShutdownRecv)>,
    dedup: bool,
}

impl Pipeline {
    #[must_use]
    pub fn builder() -> PipelineBuilder {
        PipelineBuilder::default()
    }

    pub async fn run(&mut self) -> SiliconResult<()> {
        log::info!(
            "Starting pipeline. num_datasources: {}, num_account_pipes: {}, num_block_meta_pipes: {}, num_instruction_pipes: {}",
            self.datasources.len(),
            self.account_pipes.len(),
            self.block_meta_pipes.len(),
            self.instruction_pipes.len(),
        );

        let (update_sender, mut update_receiver) = mea::mpsc::unbounded::<(Update, DatasourceId)>();

        let (shutdown_send, shutdown_recv) = self
            .shutdown_pair
            .clone()
            .unwrap_or(mea::shutdown::new_pair());

        for datasource in &self.datasources {
            let datasource = datasource.clone();
            let update_sender = update_sender.clone();
            let shutdown_recv = shutdown_recv.clone();

            tokio::spawn(async move {
                if let Err(error) = datasource.consume(update_sender, shutdown_recv).await {
                    log::error!("{error}");
                }
            });
        }

        drop(update_sender);

        let mut update_deduper = self
            .dedup
            .then(|| UpdateDeduper::new(1024, Duration::from_secs(15)));

        loop {
            tokio::select! {
                () = shutdown_recv.is_shutdown() => {
                    break;
                }

                _ = tokio::signal::ctrl_c() => {
                    shutdown_send.shutdown();
                    break;
                }

                result = update_receiver.recv() => {
                    if let Ok((update, datasource_id)) = result {
                        if let Some(update_deduper) = update_deduper.as_mut()
                            && !update_deduper.insert(UpdateKey::from(&update))
                        {
                            continue;
                        }

                        if let Err(error) = self.process(&update, &datasource_id).await {
                            log::error!("Failed to process update {update:?}: {error}");
                        }
                    } else {
                        log::error!("update_sender disconnected");
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    async fn process(
        &mut self,
        update: &Update,
        datasource_id: &DatasourceId,
    ) -> SiliconResult<()> {
        match update {
            Update::Account(account_update) => {
                for account_pipe in &mut self.account_pipes {
                    let filter_result = account_pipe
                        .filters()
                        .iter()
                        .all(|filter| filter.filter_account(datasource_id, account_update));

                    if filter_result {
                        account_pipe.run(account_update).await?;
                    }
                }
            }

            Update::BlockMeta(block_meta_update) => {
                for block_meta_pipe in &mut self.block_meta_pipes {
                    let filter_result = block_meta_pipe
                        .filters()
                        .iter()
                        .all(|filter| filter.filter_block_meta(datasource_id, block_meta_update));

                    if filter_result {
                        block_meta_pipe.run(block_meta_update).await?;
                    }
                }
            }

            Update::Transaction(transaction_update) => {
                let instructions = transaction_update.instructions();

                for instruction_pipe in &mut self.instruction_pipes {
                    let mut matching_instructions = Vec::with_capacity(instructions.len());

                    for instruction in &instructions {
                        let filter_result = instruction_pipe.filters().iter().all(|filter| {
                            filter.filter_instruction(
                                datasource_id,
                                instruction,
                                transaction_update,
                            )
                        });

                        if filter_result {
                            matching_instructions.push(instruction);
                        }
                    }

                    for matching_instruction in matching_instructions {
                        instruction_pipe
                            .run(matching_instruction, transaction_update)
                            .await?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct PipelineBuilder {
    datasources: Vec<Arc<dyn Datasource>>,
    account_pipes: Vec<Box<dyn AccountPipes>>,
    block_meta_pipes: Vec<Box<dyn BlockMetaPipes>>,
    instruction_pipes: Vec<Box<dyn InstructionPipes>>,
    shutdown_pair: Option<(mea::shutdown::ShutdownSend, mea::shutdown::ShutdownRecv)>,
    dedup: bool,
}

impl PipelineBuilder {
    #[must_use]
    pub fn datasource(mut self, datasource: impl Datasource) -> Self {
        self.datasources.push(Arc::new(datasource));
        self
    }

    #[must_use]
    pub fn account_pipe<T, P>(
        mut self,
        decoder: impl AccountDecoder<AccountType = T>,
        processor: P,
        filters: Vec<Box<dyn Filter>>,
    ) -> Self
    where
        T: Send + Sync + 'static,
        P: for<'a> Processor<AccountProcessorInputType<'a, T>>,
    {
        self.account_pipes.push(
            AccountPipe {
                decoder: decoder.into(),
                processor,
                filters,
            }
            .into(),
        );
        self
    }

    #[must_use]
    pub fn block_meta_pipe<P>(mut self, processor: P, filters: Vec<Box<dyn Filter>>) -> Self
    where
        P: Processor<BlockMetaUpdate>,
    {
        self.block_meta_pipes
            .push(BlockMetaPipe { processor, filters }.into());
        self
    }

    #[must_use]
    pub fn instruction_pipe<T, P>(
        mut self,
        decoder: impl InstructionDecoder<InstructionType = T>,
        processor: P,
        filters: Vec<Box<dyn Filter>>,
    ) -> Self
    where
        T: Send + Sync + 'static,
        P: for<'a> Processor<InstructionProcessorInputType<'a, T>>,
    {
        self.instruction_pipes.push(
            InstructionPipe {
                decoder: decoder.into(),
                processor,
                filters,
            }
            .into(),
        );
        self
    }

    #[must_use]
    pub fn shutdown_pair(
        mut self,
        shutdown_pair: (mea::shutdown::ShutdownSend, mea::shutdown::ShutdownRecv),
    ) -> Self {
        self.shutdown_pair = Some(shutdown_pair);
        self
    }

    #[must_use]
    pub fn dedup(mut self, dedup: bool) -> Self {
        self.dedup = dedup;
        self
    }

    pub fn build(self) -> SiliconResult<Pipeline> {
        Ok(Pipeline {
            datasources: self.datasources,
            account_pipes: self.account_pipes,
            block_meta_pipes: self.block_meta_pipes,
            instruction_pipes: self.instruction_pipes,
            shutdown_pair: self.shutdown_pair,
            dedup: self.dedup,
        })
    }
}
