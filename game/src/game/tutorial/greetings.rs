use std::{
    path,
    time::{Duration, Instant},
};

use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use sdl2::{image::LoadTexture, render::Texture};

use crate::game::{
    common::{event_loop_common, render_common},
    game_common_context::GameCommonContext,
    game_player::{chart_player_ui::ChartPlayerUI, janggu_state_with_tick},
};

use super::display_tutorial_messages;

pub(crate) fn do_tutorial_greetings(
    common_context: &mut GameCommonContext,
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
    let mut chart_player_ui = ChartPlayerUI::new(&texture_creator);
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
        chart_player_ui.draw(&mut common_context.canvas);

        render_common(common_context);
        common_context.canvas.present();
        continue;
    }

    display_tutorial_messages(
        common_context,
        &messages,
        janggu_state_and_tutorial_start_time,
    )
}
