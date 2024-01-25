use sdl2::{render::{Canvas, Texture, TextureCreator}, image::LoadTexture, rect::Rect, pixels::Color, video::{Window, WindowContext}};

use crate::janggu::DrumPane;

struct NoteTextures<'a> {
    덩: Texture<'a>,
    덩_돌려덕:Texture<'a>,
    덩_돌려쿵:Texture<'a>,
    덩_돌려쿵덕:Texture<'a>,
    덕:Texture<'a>,
    돌려덕:Texture<'a>,
    쿵:Texture<'a>,
    돌려쿵:Texture<'a>,
}
pub struct DisplayedSongNote {
    pub(crate) distance: f64,
    pub(crate) 궁채: Option<DrumPane>,
    pub(crate) 열채: Option<DrumPane>,
}
fn load_note_textures(texture_creator: &TextureCreator<WindowContext>) -> Result<NoteTextures, String> {
    Ok(NoteTextures {
        덩: texture_creator.load_texture("assets/img/deong00.png")?,
        덩_돌려덕:texture_creator.load_texture("assets/img/deong01.png")?,
        덩_돌려쿵:texture_creator.load_texture("assets/img/deong10.png")?,
        덩_돌려쿵덕:texture_creator.load_texture("assets/img/deong11.png")?,
        덕:texture_creator.load_texture("assets/img/duck.png")?,
        돌려덕:texture_creator.load_texture("assets/img/duckr.png")?,
        쿵:texture_creator.load_texture("assets/img/kung.png")?,
        돌려쿵:texture_creator.load_texture("assets/img/kungr.png")?,
    })
}
fn get_note_size() -> (u32, u32) {
    (100, 100)
}
pub fn get_maximum_distance(viewport_width: u32) -> f64 {
    let note_width = get_note_size().0;
    return (viewport_width - note_width - 20) as f64 / note_width as f64;
}
pub fn draw_ui(canvas: &mut Canvas<Window>, notes: Vec<DisplayedSongNote>) {
    let texture_creator = canvas.texture_creator();
    let guideline = texture_creator.load_texture("assets/img/note_guideline.png")
    .expect("Failed to load note guideline image");
    let note_width = get_note_size().0;
    let note_height = get_note_size().1;

    let bg_y = canvas.viewport().height()  as i32 - 20 - (note_height as i32 + 20);
    let guideline_x = canvas.viewport().width() as i32 - note_width as i32 - 20;
    let guideline_y = bg_y + 10;
    canvas.set_draw_color(Color::RGB(200, 200, 200));
    canvas.fill_rect(Rect::new(0, 
        bg_y, 
        canvas.viewport().width(), note_height + 20)).unwrap();
    canvas.copy(&guideline, None, Rect::new(
        guideline_x,
        guideline_y,
        note_width,
        note_height
    )).unwrap();

    let note_textures = load_note_textures(&texture_creator).unwrap();
    for i in notes {
        let note_texture_option = match i.궁채 {
            Some(DrumPane::북편) => { // 쿵
                match i.열채 {
                    Some(DrumPane::북편) => Some(&note_textures.덩_돌려덕), // 돌려덕
                    Some(DrumPane::채편) => Some(&note_textures.덩), // 덕
                    _ => Some(&note_textures.쿵)
                }
            },
            Some(DrumPane::채편) => { // 돌려쿵
                match i.열채 {
                    Some(DrumPane::북편) => Some(&note_textures.덩_돌려쿵덕), // 돌려덕
                    Some(DrumPane::채편) => Some(&note_textures.덩_돌려쿵), // 덕
                    _ => Some(&note_textures.돌려쿵)
                }
            },
            _ => match i.열채 {
                Some(DrumPane::북편) => Some(&note_textures.돌려덕), // 돌려덕
                Some(DrumPane::채편) => Some(&note_textures.덕), // 덕
                _ => None
            }
        };

        if let Some(note_texture) = note_texture_option {
            canvas.copy(note_texture, None, Rect::new(
                guideline_x - (i.distance * note_width as f64) as i32,
                guideline_y,
                note_width,
                note_height
            )).unwrap();
        }
    }
}