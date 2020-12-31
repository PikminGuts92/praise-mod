use crate::shared::*;

pub fn update_realtime_positions_tempo<T>(tempo_track: &mut Vec<T>, tpq: u16)
    where T : RealtimeTempoNote {
        // Update realtime positions of tempo map events
        let mut current_pos = 0u64;
        let mut current_pos_realtime = 0.0f64;
        let mut current_mpq = 60_000_000 / 120u32; // 120 bpm

        for tempo in tempo_track.iter_mut() {
            current_pos_realtime = calculate_realtime_ms(
                tempo.get_pos(),
                current_pos,
                current_pos_realtime,
                current_mpq,
                tpq);

            // Update tempo pos
            tempo.set_pos_realtime(current_pos_realtime);

            current_pos = tempo.get_pos();
            current_mpq = tempo.get_mpq();
        }
}

pub fn update_realtime_positions<T, S>(notes: &mut Vec<T>, tempo_track: &Vec<S>, tpq: u16)
    where T : RealtimeNote, S: RealtimeTempoNote {
        // Update realtime positions
        let mut tempo_itr = tempo_track.iter().rev();
        let mut current_tempo = tempo_itr.next().unwrap();

        for note in notes.iter_mut().rev() {
            let start_pos = note.get_pos();
            let end_pos = start_pos + note.get_length();

            // Calculate realtime end position
            while current_tempo.get_pos() > end_pos {
                current_tempo = tempo_itr.next().unwrap();
            }
            let end_pos_realtime = calculate_realtime_ms(
                end_pos,
                current_tempo.get_pos(),
                current_tempo.get_pos_realtime(),
                current_tempo.get_mpq(),
                tpq);

            // Calculate realtime start position
            while current_tempo.get_pos() > start_pos {
                current_tempo = tempo_itr.next().unwrap();
            }
            let start_pos_realtime = calculate_realtime_ms(
                start_pos, current_tempo.get_pos(),
                current_tempo.get_pos_realtime(),
                current_tempo.get_mpq(),
                tpq);

            note.set_pos_realtime(start_pos_realtime);
            note.set_length_realtime(end_pos_realtime - start_pos_realtime);
        }
}

fn calculate_realtime_ms(note_pos: u64, tempo_pos: u64, tempo_pos_realtime: f64, mpq: u32, tpq: u16) -> f64 {
    let delta_ticks = note_pos - tempo_pos;

    let delta_ms = (mpq as u64 * delta_ticks) as f64 / (1_000 * tpq as u32) as f64;
    tempo_pos_realtime + delta_ms
}
