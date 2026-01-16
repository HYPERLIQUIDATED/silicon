use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum CreatorFeeOn {
    BothToken,
    OnlyToken0,
    OnlyToken1,
}
