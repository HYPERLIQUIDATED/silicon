use std::collections::HashSet;

use async_trait::async_trait;

use crate::{error::SiliconResult, update::Update};

#[async_trait]
pub trait Datasource: Send + Sync + 'static {
    async fn consume(
        &self,
        sender: mea::mpsc::UnboundedSender<(Update, DatasourceId)>,
        shutdown_recv: mea::shutdown::ShutdownRecv,
    ) -> SiliconResult<()>;

    fn id(&self) -> &DatasourceId;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DatasourceId(String);

impl std::fmt::Display for DatasourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl DatasourceId {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

pub struct DatasourceFilter(pub(crate) HashSet<DatasourceId>);

impl Default for DatasourceFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl DatasourceFilter {
    #[must_use]
    pub fn new() -> Self {
        Self(HashSet::default())
    }

    #[must_use]
    pub fn with_datasource_id(mut self, datasource_id: DatasourceId) -> Self {
        self.0.insert(datasource_id);
        self
    }
}
