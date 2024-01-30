use num_rational::Rational32;
use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use bidrum_data_struct_lib::janggu::JangguFace;

use super::timing_judge::NoteAccuracy;

struct NoteTextures<'a> {
    덩: Texture<'a>,
    덩_돌려덕: Texture<'a>,
    덩_돌려쿵: Texture<'a>,
    덩_돌려쿵덕: Texture<'a>,
    덕: Texture<'a>,
    돌려덕: Texture<'a>,
    쿵: Texture<'a>,
    돌려쿵: Texture<'a>,
}
struct AccuracyTextures<'a> {
    overchaos: Texture<'a>,
    perfect: Texture<'a>,
    great: Texture<'a>,
    good: Texture<'a>,
    bad: Texture<'a>,
    miss: Texture<'a>,
}
pub struct DisplayedSongNote {
    pub(crate) distance: f64,
    pub(crate) 궁채: Option<JangguFace>,
    pub(crate) 북채: Option<JangguFace>,
}

pub struct UIContent {
    pub(crate) accuracy: Option<NoteAccuracy>,
}

fn load_note_textures(
    texture_creator: &TextureCreator<WindowContext>,
) -> Result<NoteTextures, String> {
    Ok(NoteTextures {
        덩: texture_creator.load_texture("assets/img/deong00.png")?,
        덩_돌려덕: texture_creator.load_texture("assets/img/deong01.png")?,
        덩_돌려쿵: texture_creator.load_texture("assets/img/deong10.png")?,
        덩_돌려쿵덕: texture_creator.load_texture("assets/img/deong11.png")?,
        덕: texture_creator.load_texture("assets/img/duck.png")?,
        돌려덕: texture_creator.load_texture("assets/img/duckr.png")?,
        쿵: texture_creator.load_texture("assets/img/kung.png")?,
        돌려쿵: texture_creator.load_texture("assets/img/kungr.png")?,
    })
}

fn load_accuracy_textures(
    texture_creator: &TextureCreator<WindowContext>,
) -> Result<AccuracyTextures, String> {
    Ok(AccuracyTextures {
        overchaos: texture_creator.load_texture("assets/img/accuracy/overchaos.png")?,
        perfect: texture_creator.load_texture("assets/img/accuracy/perfect.png")?,
        great: texture_creator.load_texture("assets/img/accuracy/great.png")?,
        good: texture_creator.load_texture("assets/img/accuracy/good.png")?,
        bad: texture_creator.load_texture("assets/img/accuracy/bad.png")?,
        miss: texture_creator.load_texture("assets/img/accuracy/miss.png")?,
    })
}

/// size of note
fn get_note_size() -> (u32, u32) {
    (80, 80)
}

/// renders game play ui with notes
pub fn draw_gameplay_ui(
    canvas: &mut Canvas<Window>,
    notes: Vec<DisplayedSongNote>,
    other: UIContent,
) {
    // loads texture of judgement line
    let texture_creator = canvas.texture_creator();
    let judgement_line_texture = texture_creator
        .load_texture("assets/img/note_guideline.png")
        .expect("Failed to load note guideline image");

    // get note size
    let note_width = get_note_size().0;
    let note_height = get_note_size().1;

    // draw background where the note moves
    let background_padding = 15;
    let background_height = note_height + background_padding * 2;
    let background_y = canvas.viewport().height() as i32 - 80 - (background_height as i32);
    canvas.set_draw_color(Color::RGB(200, 200, 200));
    canvas
        .fill_rect(Rect::new(
            0,
            background_y,
            canvas.viewport().width(),
            note_height + background_padding * 2,
        ))
        .unwrap();

    // draw background border
    let background_border_height = 5 as u32;
    canvas.set_draw_color(Color::RGB(120, 120, 120));
    canvas
        .fill_rect(Rect::new(
            0,
            background_y - background_border_height as i32,
            canvas.viewport().width(),
            background_border_height,
        ))
        .unwrap();
    canvas
        .fill_rect(Rect::new(
            0,
            background_y + background_height as i32,
            canvas.viewport().width(),
            background_border_height,
        ))
        .unwrap();

    // draw judgement line
    let judgement_line_xpos = canvas.viewport().width() as i32 - note_width as i32 - 20;
    let judgeline_line_ypos = background_y + background_padding as i32;
    canvas
        .copy(
            &judgement_line_texture,
            None,
            Rect::new(
                judgement_line_xpos,
                judgeline_line_ypos,
                note_width,
                note_height,
            ),
        )
        .unwrap();

    // load textures for the notes and accuracy
    let note_textures = load_note_textures(&texture_creator).unwrap();
    let accuracy_textures = load_accuracy_textures(&texture_creator).unwrap();

    // draw notes
    for i in notes {
        // get texture for the note
        let note_texture_option = match i.궁채 {
            Some(JangguFace::북편) => match i.북채 {
                // 돌려쿵
                Some(JangguFace::북편) => Some(&note_textures.덩_돌려쿵),
                Some(JangguFace::채편) => Some(&note_textures.덩_돌려쿵덕),
                _ => Some(&note_textures.돌려쿵),
            },
            Some(JangguFace::채편) => match i.북채 {
                Some(JangguFace::북편) => Some(&note_textures.덩),
                Some(JangguFace::채편) => Some(&note_textures.덩_돌려덕),
                _ => Some(&note_textures.쿵),
            },
            _ => match i.북채 {
                Some(JangguFace::북편) => Some(&note_textures.덕),
                Some(JangguFace::채편) => Some(&note_textures.돌려덕),
                _ => None,
            },
        };

        if let Some(note_texture) = note_texture_option {
            // draw note
            canvas
                .copy(
                    note_texture,
                    None,
                    Rect::new(
                        judgement_line_xpos - (i.distance * note_width as f64) as i32,
                        judgeline_line_ypos,
                        note_width,
                        note_height,
                    ),
                )
                .unwrap();
        }
    }

    // draw note accuracy
    if let Some(accuracy) = other.accuracy {
        let accuracy_texture = match accuracy {
            NoteAccuracy::Overchaos => accuracy_textures.overchaos,
            NoteAccuracy::Perfect => accuracy_textures.perfect,
            NoteAccuracy::Great => accuracy_textures.great,
            NoteAccuracy::Good => accuracy_textures.good,
            NoteAccuracy::Bad => accuracy_textures.bad,
            NoteAccuracy::Miss => accuracy_textures.miss,
        };

        let width = 120;
        let height = (Rational32::new(
            accuracy_texture.query().height as i32 * width as i32,
            accuracy_texture.query().width as i32,
        ))
        .to_integer();
        let x = judgement_line_xpos + (note_width as i32 / 2) - (width / 2);
        let y = judgeline_line_ypos + (note_height as i32 / 2) - (height / 2);

        canvas
            .copy(
                &accuracy_texture,
                None,
                Rect::new(x, y, width as u32, height as u32),
            )
            .unwrap();
    }
}
