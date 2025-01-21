use thiserror::Error;

pub type E = Box<dyn std::error::Error + Sync + Send + 'static>;


/// Customer errors
#[derive(Debug, Error)]
pub enum InterfaceError {
    /// Customer not found
    #[error("Missing item: {0}")]
    MissingCustomer(String),

    /// Client error
    #[error("RDS failed: {0}")]
    RdsError(aws_sdk_rdsdata::Error),

    /// Parsing error
    #[error("Invalid field: {0}")]
    FromFields(String),

    /// Unknown
    #[error("Other error: {0}")]
    Other(String),
}