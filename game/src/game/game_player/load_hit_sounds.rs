use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};

pub fn load_hit_sounds() -> [StaticSoundData; 2] {
    [
        StaticSoundData::from_file(
            "assets/sound/janggu_hit/kung.wav",
            StaticSoundSettings::default(),
        )
        .expect("Failed to load kung sound"),
        StaticSoundData::from_file(
            "assets/sound/janggu_hit/deok.wav",
            StaticSoundSettings::default(),
        )
        .expect("Failed to load deok sound"),
    ]
}
