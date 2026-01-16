use std::{collections::HashMap, str::FromStr, time::Duration};

use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use silicon_core::{
    datasource::{Datasource, DatasourceId},
    error::{Error, SiliconResult},
    update::{AccountUpdate, BlockMetaUpdate, TransactionUpdate, Update},
};
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient};
use yellowstone_grpc_proto::{
    convert_from::{create_tx_meta, create_tx_versioned},
    geyser::{
        CommitmentLevel, SubscribeRequest, SubscribeRequestFilterAccounts,
        SubscribeRequestFilterBlocksMeta, SubscribeRequestFilterTransactions, SubscribeRequestPing,
        SubscribeUpdateAccount, SubscribeUpdateBlockMeta, SubscribeUpdateTransaction,
        subscribe_update::UpdateOneof,
    },
};

#[derive(Debug, Clone)]
pub struct YellowstoneGeyserGrpcClient {
    id: DatasourceId,
    endpoint: String,
    x_token: Option<String>,
    commitment_level: Option<CommitmentLevel>,
    account_filters: HashMap<String, SubscribeRequestFilterAccounts>,
    block_meta_filters: HashMap<String, SubscribeRequestFilterBlocksMeta>,
    transaction_filters: HashMap<String, SubscribeRequestFilterTransactions>,
    connect_timeout: Duration,
    timeout: Duration,
}

impl YellowstoneGeyserGrpcClient {
    #[must_use]
    pub fn new(id: DatasourceId, endpoint: String, x_token: Option<String>) -> Self {
        Self {
            id,
            endpoint,
            x_token,
            commitment_level: None,
            account_filters: HashMap::default(),
            block_meta_filters: HashMap::default(),
            transaction_filters: HashMap::default(),
            connect_timeout: Duration::from_secs(15),
            timeout: Duration::from_secs(15),
        }
    }

    #[must_use]
    pub fn with_commitment_level(mut self, commitment_level: CommitmentLevel) -> Self {
        self.commitment_level = Some(commitment_level);
        self
    }

    #[must_use]
    pub fn with_account_filters(
        mut self,
        account_filters: HashMap<String, SubscribeRequestFilterAccounts>,
    ) -> Self {
        self.account_filters = account_filters;
        self
    }

    #[must_use]
    pub fn with_block_meta_filters(
        mut self,
        block_meta_filters: HashMap<String, SubscribeRequestFilterBlocksMeta>,
    ) -> Self {
        self.block_meta_filters = block_meta_filters;
        self
    }

    #[must_use]
    pub fn with_transaction_filters(
        mut self,
        transaction_filters: HashMap<String, SubscribeRequestFilterTransactions>,
    ) -> Self {
        self.transaction_filters = transaction_filters;
        self
    }

    #[must_use]
    pub fn with_connect_timeout(mut self, connect_timeout: Duration) -> Self {
        self.connect_timeout = connect_timeout;
        self
    }

    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

#[async_trait]
impl Datasource for YellowstoneGeyserGrpcClient {
    async fn consume(
        &self,
        sender: mea::mpsc::UnboundedSender<(Update, DatasourceId)>,
        shutdown_recv: mea::shutdown::ShutdownRecv,
    ) -> SiliconResult<()> {
        loop {
            let mut geyser_grpc_client = GeyserGrpcClient::build_from_shared(self.endpoint.clone())
                .map_err(|error| Error::FailedToConsumeDatasource {
                    datasource_id: self.id.clone(),
                    message: error.to_string(),
                })?
                .x_token(self.x_token.clone())
                .map_err(|error| Error::FailedToConsumeDatasource {
                    datasource_id: self.id.clone(),
                    message: error.to_string(),
                })?
                .tls_config(ClientTlsConfig::new().with_native_roots())
                .map_err(|error| Error::FailedToConsumeDatasource {
                    datasource_id: self.id.clone(),
                    message: error.to_string(),
                })?
                .connect_timeout(self.connect_timeout)
                .timeout(self.timeout)
                .connect()
                .await
                .map_err(|error| Error::FailedToConsumeDatasource {
                    datasource_id: self.id.clone(),
                    message: error.to_string(),
                })?;

            let subscribe_request = SubscribeRequest {
                accounts: self.account_filters.clone(),
                slots: HashMap::default(),
                transactions: self.transaction_filters.clone(),
                transactions_status: HashMap::default(),
                blocks: HashMap::default(),
                blocks_meta: self.block_meta_filters.clone(),
                entry: HashMap::default(),
                commitment: self
                    .commitment_level
                    .map(|commitment_level| commitment_level as i32),
                accounts_data_slice: vec![],
                ping: None,
                from_slot: None,
            };

            let datasource_id = self.id.clone();

            tokio::select! {
                () = shutdown_recv.is_shutdown() => {
                    log::info!("Cancelling Yellowstone gRPC subscription (id: {datasource_id})");
                    return Ok(());
                }

                result = geyser_grpc_client.subscribe_with_request(Some(subscribe_request.clone())) => {
                    match result {
                        Ok((mut subscribe_tx, mut stream)) => loop {
                            tokio::select! {
                                () = shutdown_recv.is_shutdown() => {
                                    log::info!("Cancelling Yellowstone gRPC subscription (id: {datasource_id})");
                                    return Ok(());
                                }

                                maybe_result = stream.next() => {
                                    let Some(result) = maybe_result else {
                                        break;
                                    };

                                    match result {
                                        Ok(update) => match update.update_oneof {
                                            Some(UpdateOneof::Account(account_update)) => {
                                                send_subscribe_account_update(
                                                    account_update,
                                                    &sender,
                                                    &datasource_id,
                                                );
                                            }

                                            Some(UpdateOneof::BlockMeta(block_meta_update)) => {
                                                send_subscribe_block_meta_update(
                                                    &block_meta_update,
                                                    &sender,
                                                    &datasource_id,
                                                );
                                            }

                                            Some(UpdateOneof::Transaction(transaction_update)) => {
                                                send_subscribe_transaction_update(
                                                    transaction_update,
                                                    &sender,
                                                    &datasource_id,
                                                );
                                            }

                                            Some(UpdateOneof::Ping(_)) => {
                                                if let Err(error) = subscribe_tx
                                                    .send(SubscribeRequest {
                                                        ping: Some(SubscribeRequestPing { id: 1 }),
                                                        ..Default::default()
                                                    })
                                                    .await
                                                {
                                                    log::error!(
                                                        "Failed to send ping (id: {datasource_id}): {error}"
                                                    );
                                                    break;
                                                }
                                            }

                                            _ => {}
                                        },

                                        Err(error) => {
                                            log::error!("Geyser stream error (id: {datasource_id}): {error}");
                                            break;
                                        }
                                    }
                                }
                            }
                        },

                        Err(error) => {
                            log::error!("Failed to subscribe (id: {datasource_id}): {error}");
                        }
                    }
                }
            }
        }
    }

    fn id(&self) -> &DatasourceId {
        &self.id
    }
}

#[inline]
fn send_subscribe_account_update(
    subscribe_update_account: SubscribeUpdateAccount,
    sender: &mea::mpsc::UnboundedSender<(Update, DatasourceId)>,
    datasource_id: &DatasourceId,
) {
    let slot = subscribe_update_account.slot;

    if let Some(account_info) = subscribe_update_account.account {
        let Ok(account_address) = solana_address::Address::try_from(account_info.pubkey.as_slice())
        else {
            return;
        };

        let Ok(account_owner_address) =
            solana_address::Address::try_from(account_info.owner.as_slice())
        else {
            return;
        };

        let account = solana_account::Account {
            lamports: account_info.lamports,
            data: account_info.data,
            owner: account_owner_address,
            executable: account_info.executable,
            rent_epoch: account_info.rent_epoch,
        };

        if account.lamports == 0
            && account.data.is_empty()
            && account_owner_address == solana_system_interface::program::ID
        {
            return;
        }

        let Some(signature_bytes) = account_info.txn_signature else {
            return;
        };

        let Ok(signature) = solana_signature::Signature::try_from(signature_bytes.as_slice())
        else {
            return;
        };

        let update = Update::Account(Box::new(AccountUpdate {
            address: account_address,
            account,
            signature,
            slot,
        }));

        if let Err(error) = sender.send((update, datasource_id.clone())) {
            log::error!(
                "Failed to send account update (id: {datasource_id}, signature: {signature}, address: {account_address}): {error}"
            );
        }
    }
}

#[inline]
fn send_subscribe_block_meta_update(
    subscribe_update_block_meta: &SubscribeUpdateBlockMeta,
    sender: &mea::mpsc::UnboundedSender<(Update, DatasourceId)>,
    datasource_id: &DatasourceId,
) {
    let slot = subscribe_update_block_meta.slot;

    let Ok(blockhash) = solana_hash::Hash::from_str(&subscribe_update_block_meta.blockhash) else {
        return;
    };

    let block_time = subscribe_update_block_meta
        .block_time
        .map(|block_time| block_time.timestamp);

    let update = Update::BlockMeta(Box::new(BlockMetaUpdate {
        slot,
        blockhash,
        block_time,
    }));

    if let Err(error) = sender.send((update, datasource_id.clone())) {
        log::error!(
            "Failed to send block meta update (id: {datasource_id}, slot: {slot}): {error}"
        );
    }
}

#[inline]
fn send_subscribe_transaction_update(
    subscribe_update_transaction: SubscribeUpdateTransaction,
    sender: &mea::mpsc::UnboundedSender<(Update, DatasourceId)>,
    datasource_id: &DatasourceId,
) {
    let slot = subscribe_update_transaction.slot;

    if let Some(transaction_info) = subscribe_update_transaction.transaction {
        let Ok(signature) =
            solana_signature::Signature::try_from(transaction_info.signature.as_slice())
        else {
            return;
        };

        let Some(yellowstone_transaction) = transaction_info.transaction else {
            return;
        };

        let Ok(transaction) = create_tx_versioned(yellowstone_transaction) else {
            return;
        };

        let Some(yellowstone_transaction_status_meta) = transaction_info.meta else {
            return;
        };

        let Ok(transaction_status_meta) = create_tx_meta(yellowstone_transaction_status_meta)
        else {
            return;
        };

        let update = Update::Transaction(Box::new(TransactionUpdate {
            signature,
            transaction,
            transaction_status_meta,
            slot,
        }));

        if let Err(error) = sender.send((update, datasource_id.clone())) {
            log::error!(
                "Failed to send transaction update (id: {datasource_id}, signature: {signature}): {error}"
            );
        }
    }
}
