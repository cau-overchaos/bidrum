use std::{
    fs::{self, File},
    path::Path,
};

use num_rational::Rational64;
use serde::{Deserialize, Serialize};

use crate::janggu::JangguFace;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSong {
    #[serde(skip)]
    path: String,
    pub title: String,
    pub artist: String,
    pub audio_filename: String,
    pub video_filename: String,
    pub cover_image_filename: String,
    pub levels: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameNote {
    pub 궁채: Option<JangguFace>,
    pub 북채: Option<JangguFace>,
    beat_index: u64,
    tick_nomiator: i64,
    tick_denomiator: i64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GameChart {
    pub artist: String,
    pub tracks: Vec<GameNoteTrack>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GameNoteTrack {
    pub bpm: u32,
    pub delay: u64,
    pub notes: Vec<GameNote>,
}

impl GameNote {
    /// get the position of the note in unit of beat
    pub fn beat(&self) -> Rational64 {
        return Rational64::new(self.beat_index as i64, 1)
            + if self.tick_denomiator == 0 {
                Rational64::new(0, 1)
            } else {
                Rational64::new(self.tick_nomiator, self.tick_denomiator)
            };
    }
    /// calculate the timing of the note
    pub fn timing_in_ms(&self, track_bpm: u32, track_delay: u64) -> u64 {
        // bpm = beat / minute
        // minute-per-beat = 1 / bpm
        // timing-in-minute = beat * minute-per-beat
        // timing-in-millisecond = timing-in-minute (minute) * ( 60000(millisecond) / 1(minute) )
        // timing = timing-in-millisecond
        let timing = self.beat() * Rational64::new(60000, track_bpm as i64);

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
    pub fn get_position(
        &self,
        track_bpm: u32,
        track_delay: u64,
        display_bpm: u32,
        current_time_in_ms: u64,
    ) -> f64 {
        let end_time = self.timing_in_ms(track_bpm, track_delay);
        // beat_per_millisecond = (display_bpm / 60000)
        // millisecond_per_beat = 1/ beat_per_millisecond
        // speed = 1 / millisecond_per_beat
        let speed_ratio = Rational64::new(display_bpm as i64, 60000);

        // convert the ratio into floating value
        let speed = *speed_ratio.numer() as f64 / *speed_ratio.denom() as f64;

        // return the note should be how far from the judgement line
        (end_time as f64 - current_time_in_ms as f64) * speed
    }
}

impl GameSong {
    /// Get the chart of the given level
    pub fn get_chart(&self, level: u32) -> Result<GameChart, serde_json::Error> {
        let level_file_path = Path::join(Path::new(&self.path), format!("{}.json", level));
        let level_file = File::open(level_file_path).expect("Failed to open level file");

        serde_json::from_reader(level_file)
    }

    /// Get the songs in the directory
    pub fn get_songs() -> Vec<GameSong> {
        let directories = fs::read_dir(Path::new("music")).expect("Failed to read music directory");
        let mut result = Vec::<GameSong>::new();

        for i in directories {
            if let Ok(entry) = i {
                let info_file_path = Path::join(&entry.path(), Path::new("info.json"));
                if info_file_path.exists() {
                    let info_file =
                        File::open(info_file_path).expect("Failed to open info json file");
                    let mut deserialized: GameSong =
                        serde_json::from_reader(info_file).expect("Failed to parse json file");

                    // Convert the paths into the absolute path
                    deserialized.audio_filename =
                        Path::join(&entry.path(), deserialized.audio_filename)
                            .to_str()
                            .unwrap()
                            .to_string();
                    deserialized.video_filename =
                        Path::join(&entry.path(), deserialized.video_filename)
                            .to_str()
                            .unwrap()
                            .to_string();
                    deserialized.cover_image_filename =
                        Path::join(&entry.path(), deserialized.cover_image_filename)
                            .to_str()
                            .unwrap()
                            .to_string();

                    // Set the song directory path
                    deserialized.path = entry.path().to_str().unwrap().to_string();

                    result.push(deserialized);
                }
            }
        }

        result
    }
}
