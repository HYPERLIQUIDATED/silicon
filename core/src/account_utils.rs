pub fn next_account<'a>(
    iter: &mut impl Iterator<Item = &'a solana_instruction::AccountMeta>,
) -> Option<solana_address::Address> {
    Some(iter.next()?.pubkey)
}
