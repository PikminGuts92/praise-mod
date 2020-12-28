use thiserror::Error as ThisError;
use std::error::Error;

#[derive(Debug, ThisError)]
pub enum ChartParseError {
    #[error("Failed to initially parse .chart")]
    InitialParseFail,
    #[error("Failed to parse \"[Song]\" section")]
    CantParseSongSection,
    #[error("Failed to parse \"[SyncTrack]\" section")]
    CantParseSyncTrackSection,
    #[error("Failed to parse guitar/bass \"[{track_name}]\" section")]
    CantParseGuitarBassTrackSection {
        track_name: String,
    },
}