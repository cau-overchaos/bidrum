use std::{io, fs::{self, File}, path::Path};

use num_rational::Rational64;
use serde::{Deserialize, Serialize};

use crate::janggu::DrumPane;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GameSong{
    #[serde(skip)]
    path: String,
    pub(crate) title: String,
    pub(crate) artist: String,
    pub(crate) audio_filename: String,
    pub(crate) video_filename: String,
    pub(crate) cover_image_filename: String,
    pub(crate) levels: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct GameNote {
    pub(crate) 궁채: Option<DrumPane>,
    pub(crate) 열채: Option<DrumPane>,
    beat_index: u64,
    tick_nomiator: i64,
    tick_denomiator: i64,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GameLevel {
    pub(crate) artist: String,
    pub(crate) tracks: Vec<GameNoteTrack>
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GameNoteTrack {
    pub(crate) bpm: u32,
    pub(crate) delay: u64,
    pub(crate) notes: Vec<GameNote>,
}

impl GameNote {
    pub(crate) fn beat(&self) -> Rational64 {
        return Rational64::new(self.beat_index as i64,1) + if self.tick_denomiator == 0 {
            Rational64::new(0, 1)
        } else {
            Rational64::new(self.tick_nomiator, self.tick_denomiator)
        }
    }
    pub(crate) fn end_time_in_ms(&self, track_bpm: u64, track_delay: u64) -> u64 {
        let end_time = self.beat() * 
        Rational64::new(60000, track_bpm as i64);
        
        let result = (end_time.numer() / end_time.denom()) as u64 + track_delay;
        println!("beat={} => end_time{}", self.beat().to_integer(), result);
        result
    }
    pub(crate) fn get_position(&self, track_bpm: u64, track_delay: u64, display_bpm: u64, current_time_in_ms: u64) -> f64 {
        let end_time = self.end_time_in_ms(track_bpm, track_delay);
        // beat_per_millisecond = (display_bpm / 60000)
        // millisecond_per_beat = 1/ beat_per_millisecond
        // speed = 1 / millisecond_per_beat
        let speed_ratio = Rational64::new(
            display_bpm as i64,
             60000,
        );

        let speed = *speed_ratio.numer() as f64 / *speed_ratio.denom() as f64;
        let result = (end_time as f64 - current_time_in_ms as f64) * speed;
        println!("speed={} current={}, position={}", speed, current_time_in_ms, result);
        result
    }
}

impl GameSong {
    pub(crate) fn get_level(&self, level: u32) -> Result<GameLevel, serde_json::Error> {
        let level_file_path = Path::join(
            Path::new(&self.path), 
            format!("{}.json", level));
        let level_file = File::open(level_file_path).expect("Failed to open level file");

        serde_json::from_reader(level_file)
    }
    pub(crate) fn get_songs() -> Vec<GameSong> {
        let directories = fs::read_dir(Path::new("music"))
            .expect("Failed to read music directory");
        let mut result = Vec::<GameSong>::new();

        for i in directories {
            if let Ok(entry) = i {
                let info_file_path = Path::join(&entry.path(), Path::new("info.json"));
                if info_file_path.exists() {
                    let info_file = File::open(info_file_path)
                        .expect("Failed to open info json file");
                    let mut deserialized: GameSong = serde_json::from_reader(info_file)
                        .expect("Failed to parse json file");
                    deserialized.audio_filename = Path::join(&entry.path(), deserialized.audio_filename)
                        .to_str().unwrap().to_string();
                    deserialized.video_filename = Path::join(&entry.path(), deserialized.video_filename)
                        .to_str().unwrap().to_string();
                    deserialized.cover_image_filename = Path::join(&entry.path(), deserialized.cover_image_filename)
                        .to_str().unwrap().to_string();
                    deserialized.path = entry.path().to_str().unwrap().to_string();
                    println!("{:#?}", deserialized);
                    result.push(deserialized);
                }
            }
        }

        result
    }
}