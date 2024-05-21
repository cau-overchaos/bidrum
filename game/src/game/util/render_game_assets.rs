use std::path::Path;

use sdl2::{image::LoadTexture, rect::Rect, render::Canvas, video::Window};

use crate::constants::DEFAULT_IMG_PATH as IMG_PATH;

pub fn render_cover_image_at(
    canvas: &mut Canvas<Window>,
    cover_image_path: &Path,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
) {
    let texture_creator = canvas.texture_creator();
    let cover_img_texture = texture_creator
        .load_texture(cover_image_path)
        .expect("Cover image file not found");

    canvas
        .copy(&cover_img_texture, None, Some(Rect::new(x, y, w, h)))
        .expect("Cover image rendering failure");
}

pub fn render_level_image_at(
    canvas: &mut Canvas<Window>,
    level_image_num: i32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
) {
    let texture_creator = canvas.texture_creator();
    let level_img_texture = texture_creator
        .load_texture(Path::new(&format!(
            "{}/level/{}.png",
            IMG_PATH, level_image_num
        )))
        .expect("Level image file not found");

    canvas
        .copy(&level_img_texture, None, Some(Rect::new(x, y, w, h)))
        .expect("Level image rendering failure");
}
