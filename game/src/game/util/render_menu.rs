use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{BlendMode, Texture},
};

use crate::game::game_common_context::GameCommonContext;

pub(crate) struct MenuItem<'a> {
    pub(crate) label: String,
    pub(crate) texture: Texture<'a>,
}

pub(crate) struct MenuAnimation {
    pub(crate) item_move_easing: f64,
}

impl MenuAnimation {
    pub fn new() -> MenuAnimation {
        MenuAnimation {
            item_move_easing: 0.0,
        }
    }
}

fn fit_into_number_range(val: i32, max_excluding: usize) -> usize {
    let mut tmp = val % max_excluding as i32;
    while tmp < 0 {
        tmp += max_excluding as i32;
    }

    fn fit_into_number_range(val: i32, max: usize) -> usize {
        let mut tmp = val % max as i32;
        while tmp < 0 {
            tmp += max as i32;
        }
        tmp %= max as i32;

        return tmp as usize;
    }

    tmp %= max_excluding as i32;

    return tmp as usize;
}

#[macro_export]
/// Loads menu left/right wing textures
macro_rules! load_menu_texture {
    ($texture_creator:expr) => {{
        ($texture_creator)
            .load_texture("assets/img/menu/background.png")
            .expect("Failure on wing image load")
    }};
}

macro_rules! render_menu_item {
    ($context:expr,$font:expr,$item:expr,$max_font_height:expr,$item_x:expr,$item_y:expr,$item_width:expr,$item_height:expr) => {{
        $context
            .canvas
            .copy(
                &$item.texture,
                None,
                Rect::new($item_x, $item_y, $item_width, $item_width),
            )
            .expect("Menu item image copy failure");
        let texture_creator = $context.canvas.texture_creator();
        let label = texture_creator
            .create_texture_from_surface(&$font.render(&$item.label).blended(Color::WHITE).unwrap())
            .expect("Font rendering failure");
        $context
            .canvas
            .copy(
                &label,
                None,
                Rect::new(
                    $item_x + (($item_width - label.query().width) / 2) as i32,
                    $item_y
                        + $item_width as i32
                        + (($item_height - $item_width - $max_font_height) / 2) as i32,
                    label.query().width,
                    label.query().height,
                ),
            )
            .expect("Font texture copy ailure");
    }};
}

/// Draws full-width menu on center of canvas
/// Animation progress should be in [-1, 1]
pub(crate) fn draw_menu(
    menu_background_texture: &Texture,
    context: &mut GameCommonContext,
    items: &[MenuItem],
    selected_item_index: usize,
    animation: MenuAnimation,
) {
    // Get screen viewport
    let viewport = context.canvas.viewport();

    // Calculate size
    let menu_background_size = menu_background_texture.query();
    let menu_width = viewport.width();
    let menu_height = ((menu_background_size.height as f32 / menu_background_size.width as f32)
        * menu_width as f32) as u32;

    // Calculate y position
    let y = (viewport.height() - menu_height) / 2;

    // Draw background
    context.canvas.set_blend_mode(BlendMode::Blend);
    context
        .canvas
        .copy(
            &menu_background_texture,
            None,
            Rect::new(0, y as i32, menu_width, menu_height),
        )
        .expect("Menu drawing failure");

    // Calculate margin between items
    let margin_between_items = (context.dpi.1 as f32 * 0.25) as u32; // 0.25inchi in px

    // Caculate item size
    let item_height = (menu_height as f32 / 5.0 * 4.0) as u32;
    let item_width = (item_height as f32 / 4.0 * 3.0) as u32;
    let font_height = (item_height - item_width) / 3;

    // Load font
    let ttf_context = sdl2::ttf::init().expect("Failed to init ttf context");
    let font = ttf_context
        .load_font("assets/sans.ttf", font_height as u16)
        .expect("Failred to load font");

    // Draw center first
    let center_item_x: i32 = (viewport.width() / 2 - item_width / 2) as i32
        + (ezing::expo_in(animation.item_move_easing.abs())
            * animation.item_move_easing.signum()
            * (item_width + margin_between_items) as f64) as i32;
    let center_item_y: i32 = (viewport.height() / 2 - item_height / 2) as i32;

    render_menu_item!(
        context,
        font,
        &items[selected_item_index],
        font_height,
        center_item_x,
        center_item_y,
        item_width,
        item_height
    );

    // Draw side items
    'side_drawing: for i in 1.. {
        for left in [true, false] {
            let index = fit_into_number_range(
                selected_item_index as i32 + i * (if left { -1 } else { 1 }),
                items.len(),
            );

            let item_x: i32 = center_item_x
                + (item_width + margin_between_items) as i32 * i * (if left { -1 } else { 1 });
            let item_y: i32 = center_item_y;

            if item_x < -(item_width as i32) - (margin_between_items as i32)
                || item_x
                    > viewport.width() as i32 + item_width as i32 + margin_between_items as i32
            {
                break 'side_drawing;
            }

            render_menu_item!(
                context,
                font,
                &items[index],
                font_height,
                item_x,
                item_y,
                item_width,
                item_height
            );
        }
    }
}
