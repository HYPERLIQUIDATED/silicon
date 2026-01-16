use crate::datasource::DatasourceId;

#[derive(Debug, PartialEq)]
pub enum Error {
    FailedToConsumeDatasource {
        datasource_id: DatasourceId,
        message: String,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FailedToConsumeDatasource {
                datasource_id,
                message,
            } => {
                write!(f, "Failed to consume datasource {datasource_id}: {message}")
            }
        }
    }
}

impl std::error::Error for Error {}

pub type SiliconResult<T> = Result<T, Error>;
