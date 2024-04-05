use ezing::expo_out;
use num_rational::Rational32;
use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use bidrum_data_struct_lib::janggu::{JangguFace, JangguStick};

use super::timing_judge::NoteAccuracy;

struct NoteTextures<'a> {
    left_stick: Texture<'a>,
    right_stick: Texture<'a>,
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
    pub(crate) face: JangguFace,
    pub(crate) stick: JangguStick,
}

pub struct UIContent {
    pub(crate) accuracy: Option<NoteAccuracy>,
    pub(crate) accuracy_time_progress: Option<f32>,
    pub(crate) input_effect: Option<JangguFace>,
}

fn load_note_textures(
    texture_creator: &TextureCreator<WindowContext>,
) -> Result<NoteTextures, String> {
    Ok(NoteTextures {
        left_stick: texture_creator.load_texture("assets/img/note/left_stick.png")?,
        right_stick: texture_creator.load_texture("assets/img/note/right_stick.png")?,
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

pub struct GamePlayUIResources<'a> {
    judgement_line_texture: Texture<'a>,
    note_textures: NoteTextures<'a>,
    accuray_textures: AccuracyTextures<'a>,
    janggu_texture: Texture<'a>,
}

impl GamePlayUIResources<'_> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> GamePlayUIResources {
        let judgement_line_texture = texture_creator
            .load_texture("assets/img/play_ui/note_guideline.png")
            .expect("Failed to load note guideline image");
        let note_textures = load_note_textures(texture_creator).unwrap();
        let janggu_texture = texture_creator
            .load_texture("assets/img/play_ui/janggu.png")
            .expect("Failed to load janggu image");
        let accuray_textures = load_accuracy_textures(texture_creator).unwrap();

        return GamePlayUIResources {
            judgement_line_texture: judgement_line_texture,
            note_textures: note_textures,
            janggu_texture: janggu_texture,
            accuray_textures: accuray_textures,
        };
    }
}

/// renders game play ui with notes
pub fn draw_gameplay_ui(
    canvas: &mut Canvas<Window>,
    notes: Vec<DisplayedSongNote>,
    other: UIContent,
    resources: &mut GamePlayUIResources,
) {
    // loads texture of judgement line
    let judgement_line_texture = &resources.judgement_line_texture;

    // enable alpha blending
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    // load textures
    let note_textures = &resources.note_textures;
    let accuracy_textures = &mut resources.accuray_textures;
    let janggu_texture = &resources.janggu_texture;

    // get note size
    let right_stick_note_height = 80;
    let right_stick_note_width = (note_textures.right_stick.query().width as f32
        / note_textures.right_stick.query().height as f32
        * right_stick_note_height as f32) as u32;
    let left_stick_note_height = (note_textures.left_stick.query().height as f32
        * (right_stick_note_height as f32 / note_textures.right_stick.query().height as f32))
        as u32;
    let left_stick_note_width = (note_textures.left_stick.query().width as f32
        / note_textures.left_stick.query().height as f32
        * left_stick_note_height as f32) as u32;
    let max_stick_note_height = std::cmp::max(left_stick_note_height, right_stick_note_height);

    // calculate background height
    let background_padding = 15;
    let background_border_height = 5 as u32;
    let background_height_without_border = max_stick_note_height + background_padding * 2;
    let background_height_with_border =
        background_height_without_border + background_border_height * 2;

    // calculate janggu width
    let janggu_width = ((janggu_texture.query().width as f32
        / janggu_texture.query().height as f32)
        * background_height_with_border as f32) as u32;

    // draw janggu on center
    let viewport = canvas.viewport();
    canvas.copy(
        &janggu_texture,
        None,
        Rect::new(
            (viewport.width() - janggu_width) as i32 / 2,
            (viewport.height() - background_height_with_border) as i32 / 2,
            janggu_width,
            background_height_with_border,
        ),
    );

    // draw backgrounds
    let background_width = (viewport.width() - janggu_width) / 2;
    let background_alpha = if other.input_effect.is_some() {
        200
    } else {
        150
    };
    let background_y =
        (canvas.viewport().height() as i32 - (background_height_without_border as i32)) / 2;
    for background_x in [0, background_width as i32 + janggu_width as i32] {
        canvas.set_draw_color(Color::RGBA(200, 200, 200, background_alpha));
        canvas
            .fill_rect(Rect::new(
                background_x,
                background_y,
                background_width,
                background_height_without_border,
            ))
            .unwrap();

        // draw border, too
        canvas.set_draw_color(Color::RGBA(120, 120, 120, background_alpha));
        canvas
            .fill_rect(Rect::new(
                background_x,
                background_y - background_border_height as i32,
                background_width,
                background_border_height,
            ))
            .unwrap();
        canvas
            .fill_rect(Rect::new(
                background_x,
                background_y + background_height_without_border as i32,
                background_width,
                background_border_height,
            ))
            .unwrap();
    }

    // draw judgement line
    let judgement_line_height = max_stick_note_height;
    let judgement_line_padding_px = 20;
    let judgement_line_width = ((judgement_line_texture.query().width as f32
        / judgement_line_texture.query().height as f32)
        * max_stick_note_height as f32) as u32;
    let judgeline_line_ypos = background_y + background_padding as i32;
    let judgement_line_xposes = [
        background_width as i32 - judgement_line_width as i32 - judgement_line_padding_px,
        background_width as i32 + janggu_width as i32 + judgement_line_padding_px,
    ];
    for judgement_line_xpos in judgement_line_xposes {
        canvas
            .copy(
                &judgement_line_texture,
                None,
                Rect::new(
                    judgement_line_xpos,
                    judgeline_line_ypos,
                    judgement_line_width,
                    judgement_line_height,
                ),
            )
            .unwrap();
    }

    // load textures for the notes and accuracy
    let note_width_max = std::cmp::max(left_stick_note_width, right_stick_note_width);

    // draw note
    let mut draw_note = |i: &DisplayedSongNote| {
        let note_texture = match i.stick {
            JangguStick::궁채 => &note_textures.left_stick,
            JangguStick::열채 => &note_textures.right_stick,
        };
        let note_width = match i.stick {
            JangguStick::궁채 => left_stick_note_width,
            JangguStick::열채 => right_stick_note_width,
        };
        let note_height = match i.stick {
            JangguStick::궁채 => left_stick_note_height,
            JangguStick::열채 => right_stick_note_height,
        };
        let note_ypos = background_y
            + (background_height_without_border as i32 - note_height as i32) as i32 / 2;
        let note_xpos = match i.face {
            JangguFace::궁편 => {
                judgement_line_xposes[0]
                    - (i.distance * note_width_max as f64
                        - (note_width_max as f64 - note_width as f64 / 2.0))
                        as i32
            }
            JangguFace::열편 => {
                judgement_line_xposes[1]
                    + (i.distance * note_width_max as f64
                        + (note_width_max as f64 - note_width as f64 / 2.0))
                        as i32
            }
        };
        if i.distance
            < -(judgement_line_padding_px as f64 / note_width as f64)
                + match i.face {
                    JangguFace::궁편 => 1,
                    JangguFace::열편 => 0,
                } as f64
        {
            return;
        }
        // draw note
        canvas
            .copy(
                note_texture,
                None,
                Rect::new(note_xpos, note_ypos, note_width, note_height),
            )
            .unwrap();
    };

    // draw right-stick first, and then draw left-stick.
    for i in &notes {
        if matches!(i.stick, JangguStick::열채) {
            draw_note(i);
        }
    }
    for i in &notes {
        if matches!(i.stick, JangguStick::궁채) {
            draw_note(i);
        }
    }

    // draw note accuracy
    if let Some(accuracy) = other.accuracy {
        let accuracy_texture = match accuracy {
            NoteAccuracy::Overchaos => &mut accuracy_textures.overchaos,
            NoteAccuracy::Perfect => &mut accuracy_textures.perfect,
            NoteAccuracy::Great => &mut accuracy_textures.great,
            NoteAccuracy::Good => &mut accuracy_textures.good,
            NoteAccuracy::Bad => &mut accuracy_textures.bad,
            NoteAccuracy::Miss => &mut accuracy_textures.miss,
        };

        let width = 120;
        let height = (Rational32::new(
            accuracy_texture.query().height as i32 * width as i32,
            accuracy_texture.query().width as i32,
        ))
        .to_integer();
        let x = (viewport.width() - width) as i32 / 2;
        let y_start = (viewport.height() - background_height_with_border) as i32 / 2 - (height / 2);
        let y_end = y_start - height as i32 - 10;
        let y = y_start
            + ((y_end - y_start) as f32 * expo_out(other.accuracy_time_progress.unwrap())) as i32;

        accuracy_texture
            .set_alpha_mod((expo_out(other.accuracy_time_progress.unwrap()) * 255.0) as u8);

        canvas
            .copy(
                &accuracy_texture,
                None,
                Rect::new(x, y, width as u32, height as u32),
            )
            .unwrap();
    }
}
