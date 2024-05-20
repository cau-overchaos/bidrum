use std::{
    path,
    time::{Duration, Instant},
};

use bidrum_data_struct_lib::janggu::JangguFace;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use sdl2::{image::LoadTexture, render::Texture};

use crate::game::{
    common::{event_loop_common, render_common},
    game_common_context::GameCommonContext,
    game_player::{
        draw_gameplay_ui::{
            self, DisapreaingNoteEffect, GamePlayUIResources, InputEffect, UIContent,
        },
        is_input_effect_needed, janggu_state_with_tick,
    },
};

use super::display_tutorial_messages;

pub(crate) fn do_tutorial_greetings(
    common_context: &mut GameCommonContext,
    game_ui_resources: &mut GamePlayUIResources,
    janggu_state_and_tutorial_start_time: &mut (
        &mut janggu_state_with_tick::JangguStateWithTick,
        Instant,
    ),
) {
    // Load tutorial message images and sounds
    let texture_creator = common_context.canvas.texture_creator();
    let messages = [1, 2, 3, 4].map(|idx| -> (Texture, StaticSoundData) {
        return (
            texture_creator
                .load_texture(format!("assets/img/tutorial/greeting{}.png", idx))
                .expect("Greeting tutorial image asset load failure"),
            kira::sound::static_sound::StaticSoundData::from_file(
                path::Path::new(format!("assets/audio/tutorial/greeting{}.mp3", idx).as_str()),
                StaticSoundSettings::default(),
            )
            .expect("Greeting tutorial audio load failure"),
        );
    });

    let started_at = std::time::Instant::now();
    let message_start_delay = Duration::from_secs(1);
    loop {
        if started_at.elapsed() > message_start_delay {
            break;
        }
        let tick = janggu_state_and_tutorial_start_time.1.elapsed().as_millis() as i128;
        for event in common_context.event_pump.poll_iter() {
            event_loop_common(&event, &mut common_context.coins);
        }

        janggu_state_and_tutorial_start_time
            .0
            .update(common_context.read_janggu_state(), tick);

        common_context.canvas.clear();
        draw_gameplay_ui::draw_gameplay_ui(
            &mut common_context.canvas,
            vec![],
            UIContent {
                accuracy: None,
                accuracy_time_progress: None,
                input_effect: InputEffect::new(),
                overall_effect_tick: common_context.game_initialized_at.elapsed().as_millis(),
                disappearing_note_effects: DisapreaingNoteEffect::new(),
            },
            game_ui_resources,
        );
        // First message is not started
        common_context.canvas.present();
        continue;
    }

    display_tutorial_messages(
        common_context,
        game_ui_resources,
        &messages,
        janggu_state_and_tutorial_start_time,
    )
}
