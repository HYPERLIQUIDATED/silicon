#[derive(Debug, Clone)]
pub struct AccountUpdate {
    pub address: solana_address::Address,
    pub account: solana_account::Account,
    pub signature: solana_signature::Signature,
    pub slot: solana_clock::Slot,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct AccountUpdateKey {
    pub(crate) address: solana_address::Address,
    pub(crate) lamports: u64,
    pub(crate) data_fingerprint: u64,
    pub(crate) owner: solana_address::Address,
    pub(crate) executable: bool,
    pub(crate) rent_epoch: solana_clock::Epoch,
    pub(crate) signature: solana_signature::Signature,
    pub(crate) slot: solana_clock::Slot,
}

impl From<&AccountUpdate> for AccountUpdateKey {
    fn from(value: &AccountUpdate) -> Self {
        Self {
            address: value.address,
            lamports: value.account.lamports,
            data_fingerprint: fingerprint(&value.account.data),
            owner: value.account.owner,
            executable: value.account.executable,
            rent_epoch: value.account.rent_epoch,
            signature: value.signature,
            slot: value.slot,
        }
    }
}

#[inline]
fn fingerprint(data: &[u8]) -> u64 {
    use std::hash::{DefaultHasher, Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}
