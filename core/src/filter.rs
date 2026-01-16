use crate::{
    datasource::{DatasourceFilter, DatasourceId},
    update::{AccountUpdate, BlockMetaUpdate, TransactionUpdate},
};

pub trait Filter: Send + Sync + 'static {
    fn filter_account(
        &self,
        _datasource_id: &DatasourceId,
        _account_update: &AccountUpdate,
    ) -> bool {
        true
    }

    fn filter_block_meta(
        &self,
        _datasource_id: &DatasourceId,
        _block_meta_update: &BlockMetaUpdate,
    ) -> bool {
        true
    }

    fn filter_instruction(
        &self,
        _datasource_id: &DatasourceId,
        _instruction: &solana_instruction::Instruction,
        _transaction_update: &TransactionUpdate,
    ) -> bool {
        true
    }
}

impl<T: Filter> From<T> for Box<dyn Filter> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

impl Filter for DatasourceFilter {
    fn filter_account(
        &self,
        datasource_id: &DatasourceId,
        _account_update: &AccountUpdate,
    ) -> bool {
        self.0.contains(datasource_id)
    }

    fn filter_block_meta(
        &self,
        datasource_id: &DatasourceId,
        _block_meta_update: &BlockMetaUpdate,
    ) -> bool {
        self.0.contains(datasource_id)
    }

    fn filter_instruction(
        &self,
        datasource_id: &DatasourceId,
        _instruction: &solana_instruction::Instruction,
        _transaction_update: &TransactionUpdate,
    ) -> bool {
        self.0.contains(datasource_id)
    }
}
