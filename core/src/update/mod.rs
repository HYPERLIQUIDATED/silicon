pub mod account;
pub mod block_meta;
pub mod transaction;

pub use self::{account::*, block_meta::*, transaction::*};

#[derive(Debug, Clone)]
pub enum Update {
    Account(Box<AccountUpdate>),
    BlockMeta(Box<BlockMetaUpdate>),
    Transaction(Box<TransactionUpdate>),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum UpdateKey {
    Account(AccountUpdateKey),
    BlockMeta(BlockMetaUpdateKey),
    Transaction(TransactionUpdateKey),
}

impl From<&Update> for UpdateKey {
    fn from(value: &Update) -> Self {
        match value {
            Update::Account(update) => UpdateKey::Account(AccountUpdateKey::from(update.as_ref())),

            Update::BlockMeta(update) => {
                UpdateKey::BlockMeta(BlockMetaUpdateKey::from(update.as_ref()))
            }

            Update::Transaction(update) => {
                UpdateKey::Transaction(TransactionUpdateKey::from(update.as_ref()))
            }
        }
    }
}
