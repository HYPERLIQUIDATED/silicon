pub trait ArrangeAccounts: Send + Sync + 'static {
    type ArrangedAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts>;
}
