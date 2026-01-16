use async_trait::async_trait;

use crate::{error::SiliconResult, filter::Filter, processor::Processor, update::AccountUpdate};

#[derive(Debug, Clone)]
pub struct DecodedAccount<T> {
    pub lamports: u64,
    pub data: T,
    pub owner: solana_address::Address,
    pub executable: bool,
    pub rent_epoch: solana_clock::Epoch,
}

pub trait AccountDecoder: Send + Sync + 'static {
    type AccountType;

    fn decode(
        &self,
        account: &solana_account::Account,
    ) -> Option<DecodedAccount<Self::AccountType>>;
}

impl<T: AccountDecoder> From<T> for Box<dyn AccountDecoder<AccountType = T::AccountType>> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

#[derive(Debug, Clone)]
pub struct AccountProcessorInputType<'a, T> {
    pub decoded_account: &'a DecodedAccount<T>,
    pub update: &'a AccountUpdate,
}

#[async_trait]
pub(crate) trait AccountPipes: Send + Sync + 'static {
    async fn run(&mut self, account_update: &AccountUpdate) -> SiliconResult<()>;
    fn filters(&self) -> &Vec<Box<dyn Filter>>;
}

pub(crate) struct AccountPipe<T, P> {
    pub(crate) decoder: Box<dyn AccountDecoder<AccountType = T>>,
    pub(crate) processor: P,
    pub(crate) filters: Vec<Box<dyn Filter>>,
}

#[async_trait]
impl<T, P> AccountPipes for AccountPipe<T, P>
where
    T: Send + Sync + 'static,
    P: for<'a> Processor<AccountProcessorInputType<'a, T>>,
{
    async fn run(&mut self, account_update: &AccountUpdate) -> SiliconResult<()> {
        if let Some(decoded_account) = self.decoder.decode(&account_update.account) {
            self.processor
                .process(&AccountProcessorInputType {
                    decoded_account: &decoded_account,
                    update: account_update,
                })
                .await?;
        }

        Ok(())
    }

    fn filters(&self) -> &Vec<Box<dyn Filter>> {
        &self.filters
    }
}

impl<T: AccountPipes> From<T> for Box<dyn AccountPipes> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}
