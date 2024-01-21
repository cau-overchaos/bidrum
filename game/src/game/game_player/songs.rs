use std::{io, fs::{self, File}, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GameSong{
    pub(crate) title: String,
    pub(crate) artist: String,
    pub(crate) audio_filename: String,
    pub(crate) video_filename: String,
    pub(crate) cover_image_filename: String,
}

impl GameSong {
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
                    println!("{:#?}", deserialized);
                    result.push(deserialized);
                }
            }
        }

        result
    }
}