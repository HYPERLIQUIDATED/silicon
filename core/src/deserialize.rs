pub trait ArrangeAccounts: Send + Sync + 'static {
    type ArrangedAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::account_meta::AccountMeta],
    ) -> Option<Self::ArrangedAccounts>;
}
