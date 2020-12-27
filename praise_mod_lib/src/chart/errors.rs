use thiserror::Error as ThisError;
use std::error::Error;

#[derive(Debug, ThisError)]
pub enum ChartParseError {
    #[error("Failed to initially parse .chart")]
    InitialParseFail
}