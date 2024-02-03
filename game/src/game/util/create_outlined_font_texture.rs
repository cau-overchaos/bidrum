use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator, TextureValueError},
    ttf::Font,
    video::WindowContext,
};

pub fn create_outlined_font_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &mut Font<'a, 'a>,
    text: &str,
    outline_size: u16,
    font_color: Color,
    outline_color: Color,
) -> Result<Texture<'a>, TextureValueError> {
    font.set_outline_width(0);
    let surface = font
        .render(text)
        .blended(font_color)
        .expect("Font rendering failure");

    font.set_outline_width(outline_size);
    let mut outline_surface = font
        .render(text)
        .blended(outline_color)
        .expect("Font rendering failure");

    let surface_width = surface.width();
    let surface_height = surface.height();
    let outline_surface_width = outline_surface.width();
    let outline_surface_height = outline_surface.height();

    surface
        .blit(
            None,
            &mut outline_surface,
            Rect::new(
                ((outline_surface_width - surface_width) / 2) as i32,
                ((outline_surface_height - surface_height) / 2) as i32,
                surface_width,
                surface_height,
            ),
        )
        .unwrap();

    return texture_creator.create_texture_from_surface(&outline_surface);
}
