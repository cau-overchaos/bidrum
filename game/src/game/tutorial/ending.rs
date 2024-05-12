use std::{path, time::Instant};

use kira::sound::static_sound::StaticSoundSettings;
use sdl2::image::LoadTexture;

use crate::game::{
    game_common_context::GameCommonContext,
    game_player::{draw_gameplay_ui::GamePlayUIResources, janggu_state_with_tick},
};

use super::display_tutorial_messages;

pub(crate) fn do_tutorial_ending(
    common_context: &mut GameCommonContext,
    game_ui_resources: &mut GamePlayUIResources,
    janggu_state_and_tutorial_start_time: &mut (
        &mut janggu_state_with_tick::JangguStateWithTick,
        Instant,
    ),
) {
    // Load tutorial message images and sounds
    let texture_creator = common_context.canvas.texture_creator();
    let message = (
        texture_creator
            .load_texture("assets/img/tutorial/ending.png")
            .expect("Tutorial ending image asset load failure"),
        kira::sound::static_sound::StaticSoundData::from_file(
            path::Path::new("assets/audio/tutorial/ending.mp3"),
            StaticSoundSettings::default(),
        )
        .expect("Tutorial ending audio load failure"),
    );

    display_tutorial_messages(
        common_context,
        game_ui_resources,
        &[message],
        janggu_state_and_tutorial_start_time,
    );
}
