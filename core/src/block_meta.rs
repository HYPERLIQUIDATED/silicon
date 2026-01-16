use async_trait::async_trait;

use crate::{error::SiliconResult, filter::Filter, processor::Processor, update::BlockMetaUpdate};

#[async_trait]
pub(crate) trait BlockMetaPipes: Send + Sync + 'static {
    async fn run(&mut self, block_meta_update: &BlockMetaUpdate) -> SiliconResult<()>;
    fn filters(&self) -> &Vec<Box<dyn Filter>>;
}

pub(crate) struct BlockMetaPipe<P> {
    pub(crate) processor: P,
    pub(crate) filters: Vec<Box<dyn Filter>>,
}

#[async_trait]
impl<P> BlockMetaPipes for BlockMetaPipe<P>
where
    P: Processor<BlockMetaUpdate>,
{
    async fn run(&mut self, block_meta_update: &BlockMetaUpdate) -> SiliconResult<()> {
        self.processor.process(block_meta_update).await?;

        Ok(())
    }

    fn filters(&self) -> &Vec<Box<dyn Filter>> {
        &self.filters
    }
}

impl<T: BlockMetaPipes> From<T> for Box<dyn BlockMetaPipes> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}
