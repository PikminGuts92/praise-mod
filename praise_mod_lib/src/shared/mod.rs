mod io;
mod utils;

pub use self::io::*;
pub use self::utils::*;

pub trait RealtimeNote {
    fn get_pos(&self) -> u64;
    fn get_pos_realtime(&self) -> f64;
    fn get_length(&self) -> u64;
    fn get_length_realtime(&self) -> f64;

    fn set_pos_realtime(&mut self, pos: f64);
    fn set_length_realtime(&mut self, length: f64);
}

pub trait RealtimeTempoNote: RealtimeNote {
    fn get_mpq(&self) -> u32;
    fn get_bpm(&self) -> f64;
}