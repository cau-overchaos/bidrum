use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{TargetRenderError, Texture, TextureCreator},
    ttf::Font,
    video::WindowContext,
};

use crate::game::game_common_context::GameCommonContext;

macro_rules! render_font_texture {
    ($texture_creator:expr, $font:expr, $color:expr, $text:expr) => {
        $texture_creator
            .create_texture_from_surface(
                $font
                    .render($text)
                    .blended($color)
                    .expect("Font rendering failure"),
            )
            .unwrap()
    };
}

macro_rules! create_button_texture {
    (
        $common_context: expr,
        $texture_creator: expr,
        $button_text: expr,
        $button_font: expr,
        $selected: expr) => {{
        // render text
        let button_text_font_texture =
            render_font_texture!($texture_creator, $button_font, Color::WHITE, $button_text);
        let button_text_font_texture_width = button_text_font_texture.query().width;
        let button_text_font_texture_height = button_text_font_texture.query().height;

        // paddings
        let padding_y = 10;
        let padding_x = 20;

        // calculate texture width and height
        let button_width = button_text_font_texture_width + padding_x * 2;
        let button_height = button_text_font_texture_height + padding_y * 2;

        // create empty texture
        let mut texture = $texture_creator
            .create_texture_target(
                $texture_creator.default_pixel_format(),
                button_width,
                button_height,
            )
            .unwrap();

        // make button texture
        let result = $common_context
            .canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.set_draw_color(if $selected {
                    Color::RGBA(0xf7, 0x8f, 0x07, 200)
                } else {
                    Color::RGBA(0xf7, 0xa5, 0x3b, 200)
                });
                let font_rect = Rect::new(
                    padding_x as i32,
                    padding_y as i32,
                    button_text_font_texture_width,
                    button_text_font_texture_height,
                );
                texture_canvas.clear();
                texture_canvas
                    .copy(&button_text_font_texture, None, font_rect)
                    .unwrap();
            });

        if result.is_ok() {
            Ok(texture)
        } else {
            Err(result.unwrap_err())
        }
    }};
}

pub enum DialogButton {
    Yes,
    No,
}

/// Renders dialog at center
pub fn render_confirm_dialog(
    common_context: &mut GameCommonContext,
    message: &str,
    yes_button_text: Option<&str>,
    no_button_text: Option<&str>,
    selected_button: Option<DialogButton>,
) {
    // Set font-size, line-height and load font
    let font_size = 20;
    let font_color = Color::WHITE;
    let line_height = 24;
    let font = common_context
        .ttf_context
        .load_font("assets/sans.ttf", font_size)
        .expect("Failed to load font");

    // Render lines
    let texture_creator = common_context.canvas.texture_creator();
    let font_textures: Vec<Texture> = message
        .split('\n')
        .into_iter()
        .map(|x| render_font_texture!(texture_creator, font, font_color, x))
        .collect();

    // Set blending mode
    common_context
        .canvas
        .set_blend_mode(sdl2::render::BlendMode::Blend);

    // Render button texts
    let yes_button = create_button_texture!(
        common_context,
        &texture_creator,
        yes_button_text.unwrap_or("네"),
        &font,
        matches!(selected_button, Some(DialogButton::Yes))
    )
    .expect("Failed to render yes button texture");
    let no_button = create_button_texture!(
        common_context,
        &texture_creator,
        no_button_text.unwrap_or("아니요"),
        &font,
        matches!(selected_button, Some(DialogButton::No))
    )
    .expect("Failed to render yes button texture");

    // Calculate dialog size
    let button_gap = 20;
    let gap_between_buttons_and_text = 30;
    let dialog_padding_x = 120;
    let dialog_padding_y = 20;
    let dialog_width = std::cmp::max(
        button_gap + yes_button.query().width + no_button.query().width,
        font_textures
            .iter()
            .max_by_key(|x| x.query().width)
            .unwrap()
            .query()
            .width,
    ) + dialog_padding_x * 2;
    let dialog_height = std::cmp::max(yes_button.query().height, no_button.query().height)
        + (font_textures.len() as u32) * line_height
        + gap_between_buttons_and_text
        + dialog_padding_y * 2;

    // Fill rect
    let viewport = common_context.canvas.viewport();
    let dialog_x = ((viewport.width() - dialog_width) / 2) as i32;
    let dialog_y = ((viewport.height() - dialog_height) / 2) as i32;
    common_context
        .canvas
        .set_draw_color(Color::RGBA(0xf7, 0x8f, 0x07, 180));
    common_context
        .canvas
        .fill_rect(Rect::new(dialog_x, dialog_y, dialog_width, dialog_height))
        .unwrap();

    // Copy font textures
    for (line_idx, line_texture) in font_textures.iter().enumerate() {
        common_context
            .canvas
            .copy(
                line_texture,
                None,
                Rect::new(
                    (viewport.width() - line_texture.query().width) as i32 / 2,
                    dialog_y + (dialog_padding_y + line_idx as u32 * line_height) as i32,
                    line_texture.query().width,
                    line_texture.query().height,
                ),
            )
            .expect("Failed to create dialog");
    }

    // Copy button
    let button_area_bottom = dialog_y + (dialog_height - dialog_padding_y) as i32;
    let button_area_width = yes_button.query().width + no_button.query().width + button_gap;
    common_context
        .canvas
        .copy(
            &yes_button,
            None,
            Rect::new(
                (viewport.width() - button_area_width) as i32 / 2,
                button_area_bottom - yes_button.query().height as i32,
                yes_button.query().width,
                yes_button.query().height,
            ),
        )
        .unwrap();
    common_context
        .canvas
        .copy(
            &no_button,
            None,
            Rect::new(
                (viewport.width() / 2 + button_area_width / 2 - no_button.query().width) as i32,
                button_area_bottom - no_button.query().height as i32,
                no_button.query().width,
                no_button.query().height,
            ),
        )
        .unwrap();
}
