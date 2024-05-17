mod ending;
mod greetings;
mod learn_stick_note;

use std::time::{Duration, Instant};

use bidrum_data_struct_lib::janggu::{JangguFace, JangguStick};
use kira::sound::static_sound::StaticSoundData;
use sdl2::{rect::Rect, render::Texture};

use self::{
    ending::do_tutorial_ending, greetings::do_tutorial_greetings,
    learn_stick_note::do_learn_stick_note,
};

use super::{
    common::{event_loop_common, render_common},
    game_common_context::GameCommonContext,
    game_player::{
        self,
        draw_gameplay_ui::{self, GamePlayUIResources, UIContent},
        is_input_effect_needed,
        janggu_state_with_tick::{self, JangguStateWithTick},
    },
    util::confirm_dialog::{render_confirm_dialog, DialogButton},
};

fn ask_for_tutorial(common_context: &mut GameCommonContext) -> bool {
    let ask_started_at = Instant::now();
    let mut selected = None;
    let mut janggu_state = JangguStateWithTick::new();
    'running: loop {
        let tick = ask_started_at.elapsed().as_millis();

        for i in common_context.event_pump.poll_iter() {
            if event_loop_common(&i, &mut common_context.coins) {
                return false;
            }
        }

        // break when timeout
        let elapsed_secs = ask_started_at.elapsed().as_secs();
        if elapsed_secs > 10 {
            break 'running;
        }

        // process keypress
        janggu_state.update(common_context.read_janggu_state(), tick as i128);
        if (janggu_state.궁채.is_keydown_now
            && matches!(janggu_state.궁채.face, Some(JangguFace::궁편)))
            || (janggu_state.열채.is_keydown_now
                && matches!(janggu_state.열채.face, Some(JangguFace::궁편)))
        {
            selected = Some(true);
            break;
        } else if (janggu_state.궁채.is_keydown_now
            && matches!(janggu_state.궁채.face, Some(JangguFace::열편)))
            || (janggu_state.열채.is_keydown_now
                && matches!(janggu_state.열채.face, Some(JangguFace::열편)))
        {
            selected = Some(false);
            break;
        }

        // render confirm dialog
        common_context.canvas.clear();
        render_confirm_dialog(
            common_context,
            format!(
                "튜토리얼을 진행하시겠습니까?\n남은 시간: {}",
                10 - elapsed_secs
            )
            .as_str(),
            None,
            None,
            (common_context.game_initialized_at.elapsed().as_millis() as f64 % 1000.0) / 1000.0,
        );
        render_common(common_context);
        common_context.canvas.present();
    }
    return selected.unwrap_or(true);
}

fn do_tutorial(common_context: &mut GameCommonContext) {
    let texture_creator = common_context.canvas.texture_creator();
    let mut gameplay_ui_resources =
        game_player::draw_gameplay_ui::GamePlayUIResources::new(&texture_creator);
    let mut janggu_state = janggu_state_with_tick::JangguStateWithTick::new();
    let started = std::time::Instant::now();

    // Tutorial order
    //
    // 1. left stick-left pane
    // 2. left stick-right pane
    // 3. right stick-left pane
    // 4. right stick-right pane
    // 5. mixed

    let mut janggu_state_and_start_time = (&mut janggu_state, started);

    do_tutorial_greetings(
        common_context,
        &mut gameplay_ui_resources,
        &mut janggu_state_and_start_time,
    );

    do_learn_stick_note(
        common_context,
        &mut gameplay_ui_resources,
        &mut janggu_state_and_start_time,
        JangguStick::궁채,
    );

    do_learn_stick_note(
        common_context,
        &mut gameplay_ui_resources,
        &mut janggu_state_and_start_time,
        JangguStick::열채,
    );

    do_tutorial_ending(
        common_context,
        &mut gameplay_ui_resources,
        &mut janggu_state_and_start_time,
    );
}

pub(self) fn display_tutorial_messages(
    common_context: &mut GameCommonContext,
    game_ui_resources: &mut GamePlayUIResources,
    messages: &[(Texture, StaticSoundData)],
    janggu_state_and_tutorial_start_time: &mut (&mut JangguStateWithTick, Instant),
) {
    let started_at = std::time::Instant::now();
    let mut message_started = false;
    let mut message_index = 0;
    let mut message_started_at = started_at.clone();
    let message_gap = std::time::Duration::from_secs(1);
    loop {
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
                input_effect: is_input_effect_needed(janggu_state_and_tutorial_start_time.0, tick),
            },
            game_ui_resources,
        );

        if message_index >= messages.len() {
            return;
        } else if !message_started {
            // Message will start
            common_context
                .audio_manager
                .play(messages[message_index].1.clone())
                .expect("Tutorial greeting audio play failure");
            message_started_at = Instant::now();
            message_started = true;
        } else if message_started_at.elapsed() > messages[message_index].1.duration() + message_gap
        {
            // Start new message
            message_index += 1;
            message_started = false;
            continue;
        }

        common_context
            .canvas
            .copy(
                &messages[message_index].0,
                None,
                get_message_image_asset_dst_rect(
                    common_context.canvas.viewport(),
                    messages[message_index].0.query().width,
                    messages[message_index].0.query().height,
                ),
            )
            .expect("Tutorial greeing image asset rendering failure");

        render_common(common_context);
        common_context.canvas.present();
    }
}

pub(self) fn get_message_image_asset_dst_rect(
    viewport: Rect,
    asset_width: u32,
    asset_height: u32,
) -> Rect {
    let ratio = (viewport.height() as f32 / 2.5) / asset_height as f32;
    let new_height: u32 = ((asset_height as f32) * ratio) as u32;
    let new_width: u32 = ((asset_width as f32) * ratio) as u32;

    return Rect::new(
        ((viewport.width() - new_width) / 2) as i32,
        ((viewport.height() / 2 - new_height) / 2) as i32,
        new_width,
        new_height,
    );
}

pub(crate) fn do_tutorial_if_user_wants(common_context: &mut GameCommonContext) {
    if ask_for_tutorial(common_context) {
        do_tutorial(common_context)
    }
}
