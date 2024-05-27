use std::u8;

use sdl2::{
    pixels::{Color, PixelFormatEnum},
    render::{Texture, TextureCreator, TextureValueError},
};

fn get_text_extent(font: &cairo::FontFace, font_size: f64, text: &str) -> cairo::TextExtents {
    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 0, 0)
        .expect("Failed to create temporary cairo surface for measuring text extent");
    let cairo = cairo::Context::new(&surface)
        .expect("Failed to create temporary cairo context for measuring text extent");
    cairo.set_font_face(font);
    cairo.set_font_size(font_size);
    cairo
        .text_extents(text)
        .expect("Failed to measure text extent")
        .clone()
}

pub fn measure_text_size(
    ft_font: &cairo::freetype::face::Face,
    font_size: u16,
    text: &str,
) -> (f64, f64) {
    let font = cairo::FontFace::create_from_ft(ft_font)
        .expect("Failed to create cairo font from freetype face");
    let text_extent = get_text_extent(&font, font_size as f64, text);

    (text_extent.width(), text_extent.height())
}

pub fn create_font_texture<'a, T>(
    texture_creator: &'a TextureCreator<T>,
    ft_font: &cairo::freetype::face::Face,
    text: &str,
    font_size: u16,
    outline_size: u16,
    font_color: Color,
    outline_color: Option<Color>,
) -> Result<Texture<'a>, TextureValueError> {
    let font = cairo::FontFace::create_from_ft(ft_font)
        .expect("Failed to create cairo font from freetype face");
    let text_extent = get_text_extent(&font, font_size as f64, text);
    let (surface_width, surface_height) = (
        text_extent.width() as i32 + (outline_size as i32) * 2,
        text_extent.height() as i32 + (outline_size as i32) * 2,
    );
    let cairo_surface = {
        cairo::ImageSurface::create(cairo::Format::ARgb32, surface_width, surface_height)
            .expect("Couldn't create a surface")
    };

    // Create cairo context
    let cairo_context =
        cairo::Context::new(&cairo_surface).expect("Failed to create cairo context");

    // Set text path
    cairo_context.set_font_size(font_size as f64);
    cairo_context.set_font_face(&font);
    cairo_context.move_to(
        -text_extent.x_bearing() + outline_size as f64,
        -text_extent.y_bearing() + outline_size as f64,
    );
    cairo_context.text_path(&text);

    // Draw outline
    if outline_size != 0 {
        let outline_color = outline_color.expect("Outline color should be given");
        cairo_context.set_source_rgba(
            (outline_color.r as f64) / u8::MAX as f64,
            (outline_color.g as f64) / u8::MAX as f64,
            (outline_color.b as f64) / u8::MAX as f64,
            (outline_color.a as f64) / u8::MAX as f64,
        );
        cairo_context.set_line_width(outline_size as f64);
        cairo_context
            .stroke_preserve()
            .expect("Failed to draw font-stroke");
        cairo_context.set_line_width(0.0);
    }

    // Fill inside
    cairo_context.set_source_rgba(
        (font_color.r as f64) / u8::MAX as f64,
        (font_color.g as f64) / u8::MAX as f64,
        (font_color.b as f64) / u8::MAX as f64,
        (font_color.a as f64) / u8::MAX as f64,
    );
    cairo_context.fill().expect("Failed to render font");

    // Create surface\
    let mut surface: sdl2::surface::Surface = sdl2::surface::Surface::new(
        surface_width as u32,
        surface_height as u32,
        PixelFormatEnum::ARGB8888,
    )
    .expect("Failed to create surface");
    cairo_surface
        .with_data(|surf: &[u8]| {
            surface.with_lock_mut(|pixels| {
                pixels.copy_from_slice(surf);
            })
        })
        .unwrap();

    surface.as_texture(&texture_creator)
}
