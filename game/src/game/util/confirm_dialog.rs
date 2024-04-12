use sdl2::{pixels::Color, render::Texture};

use crate::game::game_common_context::GameCommonContext;

fn render_dialog_texture(
    common_context: &mut GameCommonContext,
    message: &str,
    ok_text: Option<&str>,
    cancel_text: Option<&str>,
    selected_ok: Option<bool>,
) {
}
