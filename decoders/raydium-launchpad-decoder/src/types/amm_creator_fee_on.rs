use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AmmCreatorFeeOn {
    QuoteToken,
    BothToken,
}
