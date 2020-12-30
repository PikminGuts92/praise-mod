use crate::shared::*;

pub fn update_realtime_positions_tempo<T>(tempo_track: &mut Vec<T>, tpq: u16)
    where T : RealtimeTempoNote {
        let mut current_pos = 0u64;
        let mut current_pos_realtime = 0.0f64;
}