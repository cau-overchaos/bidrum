mod beat_and_timing;

use std::{
    fs::{self, File},
    path::Path,
};

use num_rational::Rational64;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::janggu::{JangguFace, JangguStick};

use self::beat_and_timing::{beat, get_position, timing_in_ms};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum GameSongCategory {
    Kpop,
    TraditionalKpop,
    Jpop,
    Varierty,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameSong {
    #[serde(skip)]
    path: String,
    pub title: String,
    pub artist: String,
    pub category: GameSongCategory,
    pub audio_filename: String,
    pub video_filename: Option<String>,
    pub cover_image_filename: String,
    pub levels: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameNote {
    pub stick: JangguStick,
    beat_index: u64,
    tick_nomiator: i64,
    tick_denomiator: i64,
    #[serde(skip)]
    pub id: u64,
    #[serde(skip, default = "JangguFace::default")]
    pub face: JangguFace,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameHatNote {
    beat_index: u64,
    tick_nomiator: i64,
    tick_denomiator: i64,
    #[serde(skip)]
    pub id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameChart {
    pub artist: String,
    pub delay: u64,
    pub bpm: u32,
    pub left_face: Vec<GameNote>,
    pub right_face: Vec<GameNote>,
    #[serde(default)]
    pub hats: Vec<GameHatNote>,
}

impl GameChart {
    /// Creates chart
    pub fn to_json_string(
        artist: String,
        delay: u64,
        bpm: u32,
        left_face: Vec<GameNote>,
        right_face: Vec<GameNote>,
        hats: Vec<GameHatNote>,
    ) -> Result<String, serde_json::Error> {
        let chart = GameChart {
            artist: artist,
            delay: delay,
            bpm: bpm,
            left_face: left_face,
            right_face: right_face,
            hats: hats,
        };

        serde_json::to_string(&chart)
    }

    /// Creates example chart for tutorial
    pub fn create_example_chart_for_tutorial(
        stick: JangguStick,
        face: JangguFace,
        note_count: u64,
        note_gap_in_beat: u64,
        bpm: u32,
    ) -> GameChart {
        let notes = Vec::<GameNote>::from_iter((0..note_count).map(|idx| GameNote {
            beat_index: (idx + 1) * (note_gap_in_beat),
            face: face,
            stick: stick,
            id: idx,
            tick_denomiator: 0,
            tick_nomiator: 0,
        }));

        let chart = GameChart {
            artist: String::from("Team Overchaos"),
            delay: 0,
            bpm: bpm,
            left_face: if matches!(face, JangguFace::궁편) {
                notes.clone()
            } else {
                vec![]
            },
            right_face: if matches!(face, JangguFace::열편) {
                notes.clone()
            } else {
                vec![]
            },
            hats: vec![],
        };

        return chart;
    }
}

impl GameHatNote {
    pub fn create_raw_note(
        beat_index: u64,
        tick_nomiator: i64,
        tick_denomiator: i64,
    ) -> GameHatNote {
        return GameHatNote {
            beat_index: beat_index,
            tick_nomiator: tick_nomiator,
            tick_denomiator: tick_denomiator,
            // id is useless
            id: 0,
        };
    }

    /// get the position of the note in unit of beat
    pub fn beat(&self) -> Rational64 {
        beat(
            self.beat_index as i64,
            self.tick_nomiator,
            self.tick_denomiator,
        )
    }

    /// calculate the timing of the note
    pub fn timing_in_ms(&self, track_bpm: u32, track_delay: u64) -> u64 {
        timing_in_ms(self.beat(), track_bpm, track_delay)
    }
}

impl GameNote {
    pub fn create_raw_note(
        stick: JangguStick,
        beat_index: u64,
        tick_nomiator: i64,
        tick_denomiator: i64,
    ) -> GameNote {
        return GameNote {
            stick: stick,
            beat_index: beat_index,
            tick_nomiator: tick_nomiator,
            tick_denomiator: tick_denomiator,
            // face and id are useless
            face: JangguFace::궁편,
            id: 0,
        };
    }

    /// get the position of the note in unit of beat
    pub fn beat(&self) -> Rational64 {
        beat(
            self.beat_index as i64,
            self.tick_nomiator,
            self.tick_denomiator,
        )
    }

    /// calculate the timing of the note
    pub fn timing_in_ms(&self, track_bpm: u32, track_delay: u64) -> u64 {
        timing_in_ms(self.beat(), track_bpm, track_delay)
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
        get_position(
            self.timing_in_ms(track_bpm, track_delay),
            display_bpm,
            current_time_in_ms,
        )
    }
}

impl GameSong {
    /// Get the chart of the given level
    pub fn get_chart(&self, level: u32) -> Result<GameChart, serde_json::Error> {
        let level_file_path = Path::join(Path::new(&self.path), format!("{}.json", level));
        let level_file = File::open(level_file_path).expect("Failed to open level file");

        let mut result: Result<GameChart, serde_json::Error> = serde_json::from_reader(level_file);

        // Assign note indexes
        if let Ok(result_unwrapped) = &mut result {
            let mut note_index: u64 = 0;
            for note in &mut result_unwrapped.left_face {
                note.face = JangguFace::궁편;
                note.id = note_index;
                note_index += 1;
            }
            for note in &mut result_unwrapped.right_face {
                note.face = JangguFace::열편;
                note.id = note_index;
                note_index += 1;
            }
            for note in &mut result_unwrapped.hats {
                note.id = note_index;
                note_index += 1;
            }
        }

        return result;
    }

    pub fn get_chart_levels(&self) -> Result<Vec<u32>, std::io::Error> {
        let entries = fs::read_dir(&self.path)?;
        let pattern = Regex::new("^([0-9]+)\\.json$").unwrap();
        let mut result = Vec::<u32>::new();

        for i in entries {
            if let Ok(entry) = i {
                if let Ok(file_type) = entry.file_type() {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_str().unwrap();
                    if file_type.is_file() && pattern.is_match(file_name_str) {
                        let level_str = pattern
                            .captures(file_name_str)
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str();

                        result.push(level_str.to_string().parse::<u32>().unwrap())
                    }
                }
            }
        }

        Ok(result)
    }

    pub fn get_song(path: &Path) -> Option<GameSong> {
        let info_file_path = Path::join(path, Path::new("info.json"));
        if info_file_path.exists() {
            let info_file = File::open(info_file_path).expect("Failed to open info json file");
            let mut deserialized: GameSong =
                serde_json::from_reader(info_file).expect("Failed to parse json file");

            // Convert the paths into the absolute path
            deserialized.audio_filename = Path::join(path, deserialized.audio_filename)
                .to_str()
                .unwrap()
                .to_string();
            if let Some(video_filename) = deserialized.video_filename {
                deserialized.video_filename = Some(
                    Path::join(path, video_filename)
                        .to_str()
                        .unwrap()
                        .to_string(),
                );
            }
            deserialized.cover_image_filename = Path::join(path, deserialized.cover_image_filename)
                .to_str()
                .unwrap()
                .to_string();

            // Set the song directory path
            deserialized.path = path.to_str().unwrap().to_string();

            Some(deserialized)
        } else {
            None
        }
    }

    /// Get the songs in the directory
    pub fn get_songs() -> Vec<GameSong> {
        let directories = fs::read_dir(Path::new("music")).expect("Failed to read music directory");
        let mut result = Vec::<GameSong>::new();

        for i in directories {
            if let Ok(entry) = i {
                if let Some(song) = GameSong::get_song(&entry.path()) {
                    result.push(song);
                }
            }
        }

        result
    }
}
