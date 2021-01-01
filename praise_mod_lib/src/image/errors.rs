use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ResizeImageError {
    #[error("Can't load image from memory because of \"{text}\"")]
    CantLoadImageFromMemory {
        text: String,
    },
    #[error("Can't load image from file because of \"{text}\"")]
    CantLoadImageFromFile {
        text: String,
    },
    #[error("Can't save image because of \"{text}\"")]
    CantSaveImageToFile {
        text: String,
    },
}