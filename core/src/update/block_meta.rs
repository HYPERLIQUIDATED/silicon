#[derive(Debug, Clone)]
pub struct BlockMetaUpdate {
    pub slot: solana_clock::Slot,
    pub blockhash: solana_hash::Hash,
    pub block_time: Option<i64>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct BlockMetaUpdateKey {
    pub(crate) slot: solana_clock::Slot,
}

impl From<&BlockMetaUpdate> for BlockMetaUpdateKey {
    fn from(value: &BlockMetaUpdate) -> Self {
        Self { slot: value.slot }
    }
}
