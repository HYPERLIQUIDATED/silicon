use async_trait::async_trait;

use crate::{
    error::SiliconResult, filter::Filter, processor::Processor, update::TransactionUpdate,
};

pub trait InstructionDecoder: Send + Sync + 'static {
    type InstructionType;

    fn decode(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<Self::InstructionType>;
}

impl<T: InstructionDecoder> From<T>
    for Box<dyn InstructionDecoder<InstructionType = T::InstructionType>>
{
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

#[derive(Debug, Clone)]
pub struct InstructionProcessorInputType<'a, T> {
    pub decoded_instruction: &'a T,
    pub update: &'a TransactionUpdate,
}

#[async_trait]
pub(crate) trait InstructionPipes: Send + Sync + 'static {
    async fn run(
        &mut self,
        instruction: &solana_instruction::Instruction,
        transaction_update: &TransactionUpdate,
    ) -> SiliconResult<()>;
    fn filters(&self) -> &Vec<Box<dyn Filter>>;
}

pub(crate) struct InstructionPipe<T, P> {
    pub(crate) decoder: Box<dyn InstructionDecoder<InstructionType = T>>,
    pub(crate) processor: P,
    pub(crate) filters: Vec<Box<dyn Filter>>,
}

#[async_trait]
impl<T, P> InstructionPipes for InstructionPipe<T, P>
where
    T: Send + Sync + 'static,
    P: for<'a> Processor<InstructionProcessorInputType<'a, T>>,
{
    async fn run(
        &mut self,
        instruction: &solana_instruction::Instruction,
        transaction_update: &TransactionUpdate,
    ) -> SiliconResult<()> {
        if let Some(decoded_instruction) = self.decoder.decode(instruction) {
            self.processor
                .process(&InstructionProcessorInputType {
                    decoded_instruction: &decoded_instruction,
                    update: transaction_update,
                })
                .await?;
        }

        Ok(())
    }

    fn filters(&self) -> &Vec<Box<dyn Filter>> {
        &self.filters
    }
}

impl<T: InstructionPipes> From<T> for Box<dyn InstructionPipes> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}
