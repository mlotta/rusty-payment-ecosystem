use thiserror::Error;

/// Customer errors
#[derive(Debug, Error)]
pub enum InterfaceError {
    /// Item not found
    #[error("Missing item: {0}")]
    MissingItem(String),

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
