use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum PackCreateError {
    #[error("No ogg audio found")]
    NoAudioFound,
}