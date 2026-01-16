#[derive(Debug, Clone)]
pub struct TransactionUpdate {
    pub signature: solana_signature::Signature,
    pub transaction: solana_transaction::versioned::VersionedTransaction,
    pub transaction_status_meta: solana_transaction_status_client_types::TransactionStatusMeta,
    pub slot: solana_clock::Slot,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct TransactionUpdateKey {
    pub(crate) signature: solana_signature::Signature,
}

impl From<&TransactionUpdate> for TransactionUpdateKey {
    fn from(value: &TransactionUpdate) -> Self {
        Self {
            signature: value.signature,
        }
    }
}

impl TransactionUpdate {
    #[must_use]
    pub fn instructions(&self) -> Vec<solana_instruction::Instruction> {
        let message = &self.transaction.message;
        let transaction_status_meta = &self.transaction_status_meta;
        let mut result = Vec::with_capacity(32);

        match message {
            solana_message::VersionedMessage::Legacy(message) => {
                Self::process_instructions(
                    &message.account_keys,
                    &message.instructions,
                    transaction_status_meta.inner_instructions.as_ref(),
                    &mut result,
                    |_, index| message.is_maybe_writable(index, None),
                    |_, index| message.is_signer(index),
                );
            }

            solana_message::VersionedMessage::V0(message) => {
                let mut account_keys: Vec<solana_address::Address> = Vec::with_capacity(
                    message.account_keys.len()
                        + transaction_status_meta.loaded_addresses.writable.len()
                        + transaction_status_meta.loaded_addresses.readonly.len(),
                );

                account_keys.extend_from_slice(&message.account_keys);
                account_keys.extend_from_slice(&transaction_status_meta.loaded_addresses.writable);
                account_keys.extend_from_slice(&transaction_status_meta.loaded_addresses.readonly);

                Self::process_instructions(
                    &account_keys,
                    &message.instructions,
                    transaction_status_meta.inner_instructions.as_ref(),
                    &mut result,
                    |account_key, _| {
                        transaction_status_meta
                            .loaded_addresses
                            .writable
                            .contains(account_key)
                    },
                    |_, index| index < message.header.num_required_signatures as usize,
                );
            }
        }

        result
    }

    #[inline]
    fn process_instructions<F1, F2>(
        account_keys: &[solana_address::Address],
        compiled_instructions: &[solana_message::compiled_instruction::CompiledInstruction],
        inner_instruction_groups: Option<
            &Vec<solana_transaction_status_client_types::InnerInstructions>,
        >,
        result: &mut Vec<solana_instruction::Instruction>,
        is_writable: F1,
        is_signer: F2,
    ) where
        F1: Fn(&solana_address::Address, usize) -> bool,
        F2: Fn(&solana_address::Address, usize) -> bool,
    {
        for (index, compiled_instruction) in compiled_instructions.iter().enumerate() {
            result.push(Self::build_instruction(
                account_keys,
                compiled_instruction,
                &is_writable,
                &is_signer,
            ));

            if let Some(inner_instruction_groups) = inner_instruction_groups {
                for inner_instructions in inner_instruction_groups {
                    if inner_instructions.index as usize == index {
                        for inner_instruction in &inner_instructions.instructions {
                            result.push(Self::build_instruction(
                                account_keys,
                                &inner_instruction.instruction,
                                &is_writable,
                                &is_signer,
                            ));
                        }
                    }
                }
            }
        }
    }

    #[inline]
    fn build_instruction<F1, F2>(
        account_keys: &[solana_address::Address],
        compiled_instruction: &solana_message::compiled_instruction::CompiledInstruction,
        is_writable: &F1,
        is_signer: &F2,
    ) -> solana_instruction::Instruction
    where
        F1: Fn(&solana_address::Address, usize) -> bool,
        F2: Fn(&solana_address::Address, usize) -> bool,
    {
        let program_id = account_keys
            .get(compiled_instruction.program_id_index as usize)
            .copied()
            .unwrap_or_default();

        let mut accounts = Vec::with_capacity(compiled_instruction.accounts.len());

        for account_index in &compiled_instruction.accounts {
            if let Some(account_key) = account_keys.get(*account_index as usize) {
                accounts.push(solana_instruction::account_meta::AccountMeta {
                    pubkey: *account_key,
                    is_writable: is_writable(account_key, *account_index as usize),
                    is_signer: is_signer(account_key, *account_index as usize),
                });
            }
        }

        solana_instruction::Instruction {
            program_id,
            accounts,
            data: compiled_instruction.data.clone(),
        }
    }
}
