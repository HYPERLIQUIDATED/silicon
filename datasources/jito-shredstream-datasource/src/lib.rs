use async_trait::async_trait;
use silicon_core::{
    datasource::{Datasource, DatasourceId},
    error::{Error, SiliconResult},
    update::{TransactionUpdate, Update},
};
use silicon_jito_protos::shredstream::{
    SubscribeEntriesRequest, shredstream_proxy_client::ShredstreamProxyClient,
};
use solana_entry::entry::Entry;
use solana_message::v0::MessageAddressTableLookup;
use solana_transaction_status_client_types::TransactionStatusMeta;

#[derive(Debug, Clone)]
pub struct JitoShredstreamClient {
    id: DatasourceId,
    endpoint: String,
}

impl JitoShredstreamClient {
    #[must_use]
    pub fn new(id: DatasourceId, endpoint: String) -> Self {
        Self { id, endpoint }
    }
}

#[async_trait]
impl Datasource for JitoShredstreamClient {
    async fn consume(
        &self,
        sender: mea::mpsc::UnboundedSender<(Update, DatasourceId)>,
        shutdown_recv: mea::shutdown::ShutdownRecv,
    ) -> SiliconResult<()> {
        loop {
            let mut shredstream_proxy_client =
                ShredstreamProxyClient::connect(self.endpoint.clone())
                    .await
                    .map_err(|error| Error::FailedToConsumeDatasource {
                        datasource_id: self.id.clone(),
                        message: error.to_string(),
                    })?;

            let datasource_id = self.id.clone();

            tokio::select! {
                () = shutdown_recv.is_shutdown() => {
                    log::info!("Cancelling Jito shredstream subscription (id: {datasource_id})");
                    return Ok(());
                }

                result = shredstream_proxy_client.subscribe_entries(SubscribeEntriesRequest {}) => {
                    match result {
                        Ok(response) => {
                            let mut stream = response.into_inner();

                            loop {
                                tokio::select! {
                                    () = shutdown_recv.is_shutdown() => {
                                        log::info!("Cancelling Jito shredstream subscription (id: {datasource_id})");
                                        return Ok(());
                                    }

                                    result = stream.message() => {
                                        match result {
                                            Ok(Some(entry)) => {
                                                let entries = match wincode::deserialize::<Vec<Entry>>(&entry.entries) {
                                                    Ok(entries) => entries,
                                                    Err(error) => {
                                                        log::error!("Failed to deserialize entries (id: {datasource_id}): {error}");
                                                        continue;
                                                    }
                                                };

                                                let slot = entry.slot;

                                                for entry in entries {
                                                    for transaction in entry.transactions {
                                                        if transaction
                                                            .message
                                                            .address_table_lookups()
                                                            .is_none_or(<[MessageAddressTableLookup]>::is_empty)
                                                            && let Some(&signature) = transaction.signatures.first()
                                                        {
                                                            let update = Update::Transaction(Box::new(TransactionUpdate {
                                                                signature,
                                                                transaction,
                                                                transaction_status_meta: TransactionStatusMeta {
                                                                    status: Ok(()),
                                                                    ..Default::default()
                                                                },
                                                                slot,
                                                            }));

                                                            if let Err(error) = sender.send((update, datasource_id.clone())) {
                                                                log::error!(
                                                                    "Failed to send transaction update (id: {datasource_id}, signature: {signature}): {error}"
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            Ok(None) => {
                                                log::error!("Shredstream closed (id: {datasource_id})");
                                                break;
                                            }

                                            Err(error) => {
                                                log::error!("Shredstream error (id: {datasource_id}): {error}");
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }

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
