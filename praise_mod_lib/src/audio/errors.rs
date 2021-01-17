use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum AudioReaderError {
    #[error("Can't open audio file because of \"{text}\"")]
    CantOpenAudioFile {
        text: String,
    },
    #[error("Can't decode audio file because of \"{text}\"")]
    CantDecodeAudioFile {
        text: String,
    },
}