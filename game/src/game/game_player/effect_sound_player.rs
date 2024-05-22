use kira::{
    manager::AudioManager,
    sound::{
        static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings},
        PlaybackState,
    },
};

use crate::{constants::DEFAULT_SOUND_PATH as SOUND_PATH, game};

use super::{game_result::GameResult, janggu_state_with_tick::JangguStateWithTick};

fn load_hit_sounds() -> [StaticSoundData; 2] {
    [
        StaticSoundData::from_file(
            SOUND_PATH.to_owned() + "/janggu_hit/kung.wav",
            StaticSoundSettings::default(),
        )
        .expect("Failed to load kung sound"),
        StaticSoundData::from_file(
            SOUND_PATH.to_owned() + "/janggu_hit/deok.wav",
            StaticSoundSettings::default(),
        )
        .expect("Failed to load deok sound"),
    ]
}

fn load_combo_sounds() -> [StaticSoundData; 10] {
    [
        StaticSoundData::from_file(
            SOUND_PATH.to_owned() + "/janggu_hit/Chu-imsaebyAnSukseon_5.wav",
            StaticSoundSettings::default(),
        )
        .expect("Failed to load combo sound"),
        StaticSoundData::from_file(
            SOUND_PATH.to_owned() + "/janggu_hit/Chu-imsaebyJeongSunim_2.wav",
            StaticSoundSettings::default(),
        )
        .expect("Failed to load combo sound"),
        StaticSoundData::from_file(
            SOUND_PATH.to_owned() + "/janggu_hit/Chu-imsaebyJungHoeseok_1.wav",
            StaticSoundSettings::default(),
        )
        .expect("Failed to load combo sound"),
        StaticSoundData::from_file(
            SOUND_PATH.to_owned() + "/janggu_hit/Chu-imsaebyJungHoeseok_2.wav",
            StaticSoundSettings::default(),
        )
        .expect("Failed to load combo sound"),
        StaticSoundData::from_file(
            SOUND_PATH.to_owned() + "/janggu_hit/Chu-imsaebyJungHoeseok_5.wav",
            StaticSoundSettings::default(),
        )
        .expect("Failed to load combo sound"),
        StaticSoundData::from_file(
            SOUND_PATH.to_owned() + "/janggu_hit/Chu-imsaebyJungHoeseok_6.wav",
            StaticSoundSettings::default(),
        )
        .expect("Failed to load combo sound"),
        StaticSoundData::from_file(
            SOUND_PATH.to_owned() + "/janggu_hit/Chu-imsaebyLeeChunhui_1.wav",
            StaticSoundSettings::default(),
        )
        .expect("Failed to load combo sound"),
        StaticSoundData::from_file(
            SOUND_PATH.to_owned() + "/janggu_hit/Chu-imsaebyLeeChunhui_3.wav",
            StaticSoundSettings::default(),
        )
        .expect("Failed to load combo sound"),
        StaticSoundData::from_file(
            SOUND_PATH.to_owned() + "/janggu_hit/Chu-imsaebyLeeChunhui_6.wav",
            StaticSoundSettings::default(),
        )
        .expect("Failed to load combo sound"),
        StaticSoundData::from_file(
            SOUND_PATH.to_owned() + "/janggu_hit/Chu-imsaebyLeeChunhui_7.wav",
            StaticSoundSettings::default(),
        )
        .expect("Failed to load combo sound"),
    ]
}

struct EffectSoundHandles {
    left_stick: Option<StaticSoundHandle>,
    right_stick: Option<StaticSoundHandle>,
}

impl EffectSoundHandles {
    pub fn new() -> EffectSoundHandles {
        EffectSoundHandles {
            left_stick: None,
            right_stick: None,
        }
    }
}
pub struct EffectSoundPlayer {
    effect_sound_play_handles: EffectSoundHandles,
    hit_sounds: [StaticSoundData; 2],
    combo_sounds: [StaticSoundData; 10],
    combo_sound_played: bool,
}

impl EffectSoundPlayer {
    pub fn new() -> EffectSoundPlayer {
        EffectSoundPlayer {
            effect_sound_play_handles: EffectSoundHandles::new(),
            hit_sounds: load_hit_sounds(),
            combo_sounds: load_combo_sounds(),
            combo_sound_played: false,
        }
    }

    pub fn play_janggu_sound(
        &mut self,
        janggu_state_with_tick: &JangguStateWithTick,
        audio_manager: &mut AudioManager,
    ) {
        let kung_sound_data = self.hit_sounds[0].clone();
        let deok_sound_data = self.hit_sounds[1].clone();

        if janggu_state_with_tick.궁채.is_keydown_now {
            let play_sound = if let Some(handle) = &mut self.effect_sound_play_handles.left_stick {
                !matches!(handle.state(), PlaybackState::Playing) || handle.position() > 0.01
            } else {
                true
            };
            if play_sound {
                let new_handle = audio_manager
                    .play(kung_sound_data.clone())
                    .expect("Failed to play kung sound");
                self.effect_sound_play_handles.left_stick = Some(new_handle);
            }
        }
        if janggu_state_with_tick.열채.is_keydown_now {
            let play_sound = if let Some(handle) = &mut self.effect_sound_play_handles.right_stick {
                !matches!(handle.state(), PlaybackState::Playing) || handle.position() > 0.01
            } else {
                true
            };

            if play_sound {
                let new_handle = audio_manager
                    .play(deok_sound_data.clone())
                    .expect("Failed to play deok sound");
                self.effect_sound_play_handles.right_stick = Some(new_handle);
            }
        }
    }

    pub fn play_combo_sound(&mut self, game_result: &GameResult, audio_manager: &mut AudioManager) {
        let combo_sound = self.combo_sounds[0].clone();

        if game_result.combo > 0 && (game_result.combo % 10 == 0) {
            if !self.combo_sound_played {
                let new_handle = audio_manager
                    .play(combo_sound.clone())
                    .expect("Failed to play combo sound");
                self.combo_sound_played = true;
            }
        } else {
            self.combo_sound_played = false;
        }
    }
}
