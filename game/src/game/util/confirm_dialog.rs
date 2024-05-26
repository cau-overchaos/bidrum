use sdl2::{
    image::LoadTexture,
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::Texture,
};

use crate::constants::DEFAULT_FONT_PATH as FONT_PATH;
use crate::game::util::create_outlined_font_texture::measure_text_size;
use crate::{
    constants::DEFAULT_DIALOG_PATH as DIALOG_PATH,
    game::util::create_outlined_font_texture::create_font_texture,
};

use crate::game::game_common_context::GameCommonContext;

macro_rules! create_button_texture {
    (
        $button_type: expr,
        $texture_creator: expr,
        $button_text: expr,
        $button_font: expr,
        $button_font_size: expr,
        $selected: expr) => {{
        if $button_type != "left" && $button_type != "right" {
            panic!("button type should be \"left\" or \"right\"");
        }

        // get text size
        let font_size = measure_text_size($button_font, $button_font_size, $button_text);
        let button_text_font_texture_width = font_size.0 as u32;
        let button_text_font_texture_height = font_size.1 as u32;

        // load button background texture
        let button_background_filename = format!("{}/{}_button.png", DIALOG_PATH, $button_type);
        let button_size = {
            let tmp = $texture_creator
                .load_texture(button_background_filename.clone())
                .expect("Failed to find dialog button background image");
            (tmp.query().width, tmp.query().height)
        };

        // calculate texture width and height
        let button_width = button_size.0;
        let button_height = button_size.1;

        // create empty surface
        let mut surface = sdl2::surface::Surface::new(
            button_width,
            button_height,
            sdl2::pixels::PixelFormatEnum::RGBA32,
        )
        .unwrap();
        surface
            .set_blend_mode(sdl2::render::BlendMode::Blend)
            .unwrap();
        let mut surface_canvas = sdl2::render::Canvas::from_surface(surface).unwrap();
        let surface_texture_creator = surface_canvas.texture_creator();

        // Load button background
        let button_background = surface_texture_creator
            .load_texture(button_background_filename)
            .unwrap();

        // Render font
        let button_text_font_texture = create_font_texture(
            &surface_texture_creator,
            &$button_font,
            $button_text,
            $button_font_size,
            0,
            Color::WHITE,
            None,
        )
        .expect("Failed to render font");

        // make button texture
        let font_rect = Rect::new(
            (button_width - button_text_font_texture_width) as i32 / 2
                + if $button_type == "left" { -10 } else { 10 },
            (button_height - button_text_font_texture_height) as i32 / 2,
            button_text_font_texture_width,
            button_text_font_texture_height,
        );
        surface_canvas.set_blend_mode(sdl2::render::BlendMode::None);
        surface_canvas.set_draw_color(Color::RGBA(0, 255, 0, 0));
        surface_canvas.clear();
        surface_canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        surface_canvas.copy(&button_background, None, None).unwrap();
        surface_canvas
            .copy(&button_text_font_texture, None, font_rect)
            .unwrap();

        let result = $texture_creator.create_texture_from_surface(surface_canvas.into_surface());

        result
    }};
}

/// Renders dialog at center
pub fn render_confirm_dialog(
    common_context: &mut GameCommonContext,
    message: &str,
    yes_button_text: Option<&str>,
    no_button_text: Option<&str>,
    animation_progress: f64,
    alpha: Option<u8>,
) {
    // Set font-size, line-height and load font
    let font_size = 38;
    let button_font_size = 32;
    let font_color = Color::WHITE;
    let line_height = 40;
    let font = common_context
        .freetype_library
        .new_face(FONT_PATH.to_owned() + "/sans.ttf", 327680 /* Medium */)
        .expect("Failed to load font");
    let button_font = common_context
        .freetype_library
        .new_face(FONT_PATH.to_owned() + "/sans.ttf", 327680 /* Medium */)
        .expect("Failed to load font");

    let texture_creator = common_context.canvas.texture_creator();
    let mut font_textures: Vec<Texture> = message
        .split('\n')
        .into_iter()
        .map(|x| {
            create_font_texture(&texture_creator, &font, x, font_size, 0, font_color, None)
                .expect("Failed to render text")
        })
        .collect();

    // Set blending mode
    common_context
        .canvas
        .set_blend_mode(sdl2::render::BlendMode::Blend);

    // Render button texts
    let yes_button = create_button_texture!(
        "left",
        &texture_creator,
        yes_button_text.unwrap_or("네"),
        &button_font,
        button_font_size,
        matches!(selected_button, Some(DialogButton::Yes))
    )
    .expect("Failed to render yes button texture");
    let no_button = create_button_texture!(
        "right",
        &texture_creator,
        no_button_text.unwrap_or("아니요"),
        &button_font,
        button_font_size,
        matches!(selected_button, Some(DialogButton::No))
    )
    .expect("Failed to render yes button texture");

    // Load janggu icon
    let janggu_icon = texture_creator
        .load_texture(DIALOG_PATH.to_owned() + "/janggu.png")
        .expect("Failed to load janggu image");

    // Declare button gap
    let max_button_gap = 60;
    let min_button_gap = 30;
    let button_gap = min_button_gap
        + ((max_button_gap - min_button_gap) as f64
            * ezing::sine_inout(if animation_progress > 0.5 {
                (1.0 - animation_progress) * 2.0
            } else {
                animation_progress * 2.0
            })) as u32;
    assert!(min_button_gap <= button_gap && button_gap <= max_button_gap);

    // Render button area
    let mut button_area = texture_creator
        .create_texture_from_surface({
            // Calculate width and height
            let width = button_gap * 2
                + yes_button.query().width
                + no_button.query().width
                + janggu_icon.query().width;
            let height = *([
                yes_button.query().height,
                no_button.query().height,
                janggu_icon.query().height,
            ]
            .iter()
            .max()
            .unwrap());

            // Create surface
            let mut surface = sdl2::surface::Surface::new(width, height, PixelFormatEnum::RGBA32)
                .expect("Failed to create surface");
            surface
                .set_blend_mode(sdl2::render::BlendMode::Blend)
                .unwrap();
            let mut surface_canvas = sdl2::render::Canvas::from_surface(surface).unwrap();
            let texture_creator = surface_canvas.texture_creator();

            // Load required textures again
            let yes_button = create_button_texture!(
                "left",
                &texture_creator,
                yes_button_text.unwrap_or("네"),
                &button_font,
                button_font_size,
                matches!(selected_button, Some(DialogButton::Yes))
            )
            .expect("Failed to render yes button texture");
            let no_button = create_button_texture!(
                "right",
                &texture_creator,
                no_button_text.unwrap_or("아니요"),
                &button_font,
                button_font_size,
                matches!(selected_button, Some(DialogButton::No))
            )
            .expect("Failed to render yes button texture");
            let janggu_icon = texture_creator
                .load_texture(DIALOG_PATH.to_owned() + "/janggu.png")
                .expect("Failed to load janggu image");

            // Copy
            surface_canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
            surface_canvas
                .copy(
                    &yes_button,
                    None,
                    Rect::new(
                        0,
                        (height - yes_button.query().height) as i32 / 2,
                        yes_button.query().width,
                        yes_button.query().height,
                    ),
                )
                .unwrap();
            surface_canvas
                .copy(
                    &no_button,
                    None,
                    Rect::new(
                        (width - no_button.query().width) as i32,
                        (height - no_button.query().height) as i32 / 2,
                        no_button.query().width,
                        no_button.query().height,
                    ),
                )
                .unwrap();
            surface_canvas
                .copy(
                    &janggu_icon,
                    None,
                    Rect::new(
                        (width - janggu_icon.query().width) as i32 / 2,
                        (height - janggu_icon.query().height) as i32 / 2,
                        janggu_icon.query().width,
                        janggu_icon.query().height,
                    ),
                )
                .unwrap();

            surface_canvas.into_surface()
        })
        .expect("Failed to create button area texture");

    // Calculate dialog size
    let gap_between_buttons_and_text = 120;
    let text_padding_top = 120;
    let dialog_padding_x = 60;
    let dialog_padding_y = 40;
    let dialog_width = std::cmp::max(
        max_button_gap * 2
            + janggu_icon.query().width
            + yes_button.query().width
            + no_button.query().width,
        font_textures
            .iter()
            .max_by_key(|x| x.query().width)
            .unwrap()
            .query()
            .width,
    ) + dialog_padding_x * 2;
    let dialog_height = std::cmp::max(yes_button.query().height, no_button.query().height)
        + (font_textures.len() as u32) * line_height
        + text_padding_top
        + gap_between_buttons_and_text
        + dialog_padding_y * 2;

    // Draw border
    let border_size = 3;
    let viewport = common_context.canvas.viewport();
    let dialog_x = ((viewport.width() - dialog_width) / 2) as i32;
    let dialog_y = ((viewport.height() - dialog_height) / 2) as i32;
    common_context
        .canvas
        .set_draw_color(Color::RGBA(0x62, 0x62, 0x62, alpha.unwrap_or(255)));
    common_context
        .canvas
        .fill_rect(Rect::new(
            dialog_x - border_size,
            dialog_y - border_size,
            dialog_width + border_size as u32 * 2,
            dialog_height + border_size as u32 * 2,
        ))
        .unwrap();

    // Copy background
    let mut background = texture_creator
        .load_texture(DIALOG_PATH.to_owned() + "/bg.png")
        .expect("Failed to load background texture");
    background.set_alpha_mod(alpha.unwrap_or(255));
    common_context
        .canvas
        .copy(
            &background,
            None,
            Rect::new(dialog_x, dialog_y, dialog_width, dialog_height),
        )
        .unwrap();

    // Copy font textures
    for (line_idx, line_texture) in font_textures.iter_mut().enumerate() {
        if let Some(alpha) = alpha {
            line_texture.set_alpha_mod(alpha);
        }

        common_context
            .canvas
            .copy(
                line_texture,
                None,
                Rect::new(
                    (viewport.width() - line_texture.query().width) as i32 / 2,
                    dialog_y
                        + text_padding_top as i32
                        + (dialog_padding_y + line_idx as u32 * line_height) as i32,
                    line_texture.query().width,
                    line_texture.query().height,
                ),
            )
            .expect("Failed to create dialog");
    }

    // Copy button
    let button_area_bottom = dialog_y + (dialog_height - dialog_padding_y) as i32;
    let button_area_y = button_area_bottom - button_area.query().height as i32;
    if let Some(alpha) = alpha {
        button_area.set_alpha_mod(alpha);
    }

    common_context
        .canvas
        .copy(
            &button_area,
            None,
            Rect::new(
                (viewport.width() - button_area.query().width) as i32 / 2,
                button_area_y,
                button_area.query().width,
                button_area.query().height,
            ),
        )
        .unwrap();
}
