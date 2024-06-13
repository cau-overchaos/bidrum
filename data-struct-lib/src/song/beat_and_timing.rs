use num_rational::Rational64;

/// get the position of the note in unit of beat
pub(super) fn beat(beat_index: i64, tick_nomiator: i64, tick_denomiator: i64) -> Rational64 {
    return Rational64::new(beat_index as i64, 1)
        + if tick_denomiator == 0 {
            Rational64::new(0, 1)
        } else {
            Rational64::new(tick_nomiator, tick_denomiator)
        };
}

/// calculate the timing of the note
pub(super) fn timing_in_ms(beat: Rational64, track_bpm: u32, track_delay: u64) -> u64 {
    // bpm = beat / minute
    // minute-per-beat = 1 / bpm
    // timing-in-minute = beat * minute-per-beat
    // timing-in-millisecond = timing-in-minute (minute) * ( 60000(millisecond) / 1(minute) )
    // timing = timing-in-millisecond
    let timing = beat * Rational64::new(60000, track_bpm as i64);

    (timing.numer() / timing.denom()) as u64 + track_delay
}

/// Get the position of the note in the display.
/// In other words, get the note should be how far from the judgement line
/// in unit of the note width.
///
/// # Return value example
///    - `-1.0` : after the judgement line the width of the note
///    - `0.0` : at the judgement line
///    - `1.0` : before the judgement line the width of the note
///    - `2.0` : before the judgement line the width of the two notes
pub(super) fn get_position(timinig_in_ms: u64, display_bpm: u32, current_time_in_ms: u64) -> f64 {
    let end_time = timinig_in_ms;
    // beat_per_millisecond = (display_bpm / 60000)
    // millisecond_per_beat = 1/ beat_per_millisecond
    // speed = 1 / millisecond_per_beat
    let speed_ratio = Rational64::new(display_bpm as i64, 60000);

    // convert the ratio into floating value
    let speed = *speed_ratio.numer() as f64 / *speed_ratio.denom() as f64;

    // return the note should be how far from the judgement line
    (end_time as f64 - current_time_in_ms as f64) * speed
}
