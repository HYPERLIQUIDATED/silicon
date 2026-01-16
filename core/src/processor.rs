use crate::error::SiliconResult;

pub trait Processor<T: Sync>: Send + Sync + 'static {
    fn process(&mut self, data: &T) -> impl Future<Output = SiliconResult<()>> + Send;
}
