pub fn next_account<'a>(
    iter: &mut impl Iterator<Item = &'a solana_instruction::account_meta::AccountMeta>,
) -> Option<solana_address::Address> {
    Some(iter.next()?.pubkey)
}
