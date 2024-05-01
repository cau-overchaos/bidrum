use std::{path, time::Instant};

use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use sdl2::{image::LoadTexture, rect::Rect, render::Texture};

use crate::game::{
    common::{self, event_loop_common, render_common},
    game_common_context::GameCommonContext,
    game_player::draw_gameplay_ui::{self, GamePlayUIResources, UIContent},
    start,
};

use super::get_message_image_asset_dst_rect;

pub(crate) fn do_tutorial_greetings(
    common_context: &mut GameCommonContext,
    game_ui_resources: &mut GamePlayUIResources,
) {
    // Load sounds
    let sounds = [1, 2, 3, 4].map(|idx| -> StaticSoundData {
        return kira::sound::static_sound::StaticSoundData::from_file(
            path::Path::new(format!("assets/audio/tutorial/greeting{}.mp3", idx).as_str()),
            StaticSoundSettings::default(),
        )
        .expect("Greeting tutorial audio load failure");
    });

    // Load message images
    let texture_creator = common_context.canvas.texture_creator();
    let messages = [1, 2, 3, 4].map(|idx| -> Texture {
        return texture_creator
            .load_texture(format!("assets/img/tutorial/greeting{}.png", idx))
            .expect("Greeting tutorial image asset load failure");
    });

    let started_at = std::time::Instant::now();
    let mut message_started = false;
    let mut message_index = 0;
    let mut message_started_at = started_at.clone();
    let message_gap = std::time::Duration::from_secs(1);
    let message_start_delay = std::time::Duration::from_secs(1);
    loop {
        for event in common_context.event_pump.poll_iter() {
            event_loop_common(&event, &mut common_context.coins);
        }

        common_context.canvas.clear();
        draw_gameplay_ui::draw_gameplay_ui(
            &mut common_context.canvas,
            vec![],
            UIContent {
                accuracy: None,
                accuracy_time_progress: None,
                input_effect: [None, None],
            },
            game_ui_resources,
        );

        if (started_at.elapsed() <= message_start_delay) {
            // First message is not started
            common_context.canvas.present();
            continue;
        } else {
            if message_index >= 3 {
                return;
            } else if !message_started {
                // Message will start
                common_context
                    .audio_manager
                    .play(sounds[message_index].clone())
                    .expect("Tutorial greeting audio play failure");
                message_started_at = Instant::now();
                message_started = true;
            } else if message_started_at.elapsed() > sounds[message_index].duration() + message_gap
            {
                // Start new message
                message_index += 1;
                message_started = false;
                continue;
            }

            common_context
                .canvas
                .copy(
                    &messages[message_index],
                    None,
                    get_message_image_asset_dst_rect(
                        common_context.canvas.viewport(),
                        messages[message_index].query().width,
                        messages[message_index].query().height,
                    ),
                )
                .expect("Tutorial greeing image asset rendering failure");
        }

        render_common(common_context);
        common_context.canvas.present();
    }
}
