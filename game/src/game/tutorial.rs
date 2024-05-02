mod greetings;

use sdl2::rect::Rect;

use self::greetings::do_tutorial_greetings;

use super::{
    game_common_context::GameCommonContext,
    game_player::{
        self,
        janggu_state_with_tick::{self},
    },
};

fn ask_for_tutorial(_common_context: &mut GameCommonContext) -> bool {
    true
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

    do_tutorial_greetings(
        common_context,
        &mut gameplay_ui_resources,
        (&mut janggu_state, started),
    );
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
