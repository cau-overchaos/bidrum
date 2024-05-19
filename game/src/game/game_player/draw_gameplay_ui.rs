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

use super::{janggu_state_with_tick::JangguStateWithTick, timing_judge::NoteAccuracy};

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

#[derive(Clone)]
pub struct InputEffectItem {
    pub(crate) pressed: bool,
    pub(crate) keydown_timing: Option<i128>,
}

#[derive(Clone)]
pub struct InputEffect {
    pub(crate) left_face: InputEffectItem,
    pub(crate) right_face: InputEffectItem,
    pub(crate) base_tick: i128,
}

impl InputEffect {
    pub fn new() -> InputEffect {
        return InputEffect {
            base_tick: 0,
            left_face: InputEffectItem {
                pressed: false,
                keydown_timing: None,
            },
            right_face: InputEffectItem {
                pressed: false,
                keydown_timing: None,
            },
        };
    }
    pub fn update(&mut self, janggu: &JangguStateWithTick, tick_now: i128) {
        self.base_tick = tick_now;

        // Process left face
        self.left_face.pressed = false;
        if janggu.궁채.face.is_some_and(|x| x == JangguFace::궁편) {
            self.left_face.pressed = true;
            self.left_face.keydown_timing =
                Some(if let Some(prev) = self.left_face.keydown_timing {
                    prev.max(janggu.궁채.keydown_timing)
                } else {
                    janggu.궁채.keydown_timing
                })
        }
        if janggu.열채.face.is_some_and(|x| x == JangguFace::궁편) {
            self.left_face.pressed = true;
            self.left_face.keydown_timing =
                Some(if let Some(prev) = self.left_face.keydown_timing {
                    prev.max(janggu.열채.keydown_timing)
                } else {
                    janggu.열채.keydown_timing
                })
        }

        // Process right face
        self.right_face.pressed = false;
        if janggu.궁채.face.is_some_and(|x| x == JangguFace::열편) {
            self.right_face.pressed = true;
            self.right_face.keydown_timing =
                Some(if let Some(prev) = self.right_face.keydown_timing {
                    prev.max(janggu.궁채.keydown_timing)
                } else {
                    janggu.궁채.keydown_timing
                })
        }
        if janggu.열채.face.is_some_and(|x| x == JangguFace::열편) {
            self.right_face.pressed = true;
            self.right_face.keydown_timing =
                Some(if let Some(prev) = self.right_face.keydown_timing {
                    prev.max(janggu.열채.keydown_timing)
                } else {
                    janggu.열채.keydown_timing
                })
        }
    }
}

pub struct UIContent {
    pub(crate) accuracy: Option<NoteAccuracy>,
    pub(crate) accuracy_time_progress: Option<f32>,
    pub(crate) input_effect: InputEffect,
    pub(crate) overall_effect_tick: u128,
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
    let right_stick_note_height = 90;
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
    let janggu_width_min = ((janggu_texture.query().width as f32
        / janggu_texture.query().height as f32)
        * background_height_with_border as f32) as u32;
    let janggu_width_max = janggu_width_min + 20;

    // calc janggu icon size
    let janggu_width = janggu_width_min
        + ((janggu_width_max - janggu_width_min) as f64 * {
            // animation duration
            let animation_duration = 250;

            // last keydown timing
            let last_keydown_timing = other
                .input_effect
                .left_face
                .keydown_timing
                .unwrap_or(0)
                .max(other.input_effect.right_face.keydown_timing.unwrap_or(0));

            // elapsed time since last keydown timing
            let delta = other.input_effect.base_tick - last_keydown_timing;

            // return easing value
            if delta < animation_duration {
                1.0 - ezing::bounce_inout(delta as f64 / animation_duration as f64)
            } else {
                0.0
            }
        }) as u32;
    let janggu_height = ((janggu_texture.query().height as f32
        / janggu_texture.query().width as f32)
        * janggu_width as f32) as u32;

    // get viewport
    let viewport = canvas.viewport();

    // draw backgrounds
    let background_width = (viewport.width() - janggu_width_min) / 2;
    let background_y =
        (canvas.viewport().height() as i32 - (background_height_without_border as i32)) / 2;
    for background_x in [
        0,                                                 /* x coordinate of left background */
        background_width as i32 + janggu_width_min as i32, /* x coordinate of right background */
    ] {
        let background_alpha = {
            // is the face hitted?
            let hitting = if background_x == 0 {
                other.input_effect.left_face.pressed
            } else {
                other.input_effect.right_face.pressed
            };

            // base which changed by whether it's hitted or not
            let base = if hitting { 200 } else { 100 };

            // effect that changes by time
            let effect_by_time = (50.0 * {
                ezing::quad_out({
                    let blinking_duration = 300;
                    let total_duration = 1000;
                    assert!(0 < blinking_duration && blinking_duration <= total_duration);

                    let remainder = (other.overall_effect_tick % total_duration) as f64;
                    if remainder < blinking_duration as f64 {
                        (blinking_duration as f64 - remainder) / blinking_duration as f64
                    } else {
                        0.0
                    }
                })
            }) as u8;

            base + effect_by_time
        };
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
        background_width as i32 - judgement_line_width as i32 - judgement_line_padding_px, /* left judgement line */
        background_width as i32 + janggu_width_min as i32 + judgement_line_padding_px, /* right judgement line */
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

        /*
         *   note_xpos                                           judgement_line_xpos
         *      +---------                                              +----
         *      -        -          distance_between_centers            -   -
         *      -    *----------------------------------------------------* -
         *      -        -                                              -   -
         *      ----------                                              -----
         */
        let distance_between_centers = i.distance * note_width_max as f64;
        let note_xpos = (match i.face {
            JangguFace::궁편 => judgement_line_xposes[0],
            JangguFace::열편 => judgement_line_xposes[1],
        } as f64
            + (judgement_line_width / 2) as f64
            + distance_between_centers
                * match i.face {
                    JangguFace::궁편 => -1.0,
                    JangguFace::열편 => 1.0,
                }
            - (note_width / 2) as f64) as i32;

        // Do not render note if the note is on janggu icon
        let near_center_edge_x_pos = match i.face {
            JangguFace::궁편 => note_xpos + note_width as i32,
            JangguFace::열편 => note_xpos,
        };
        let distance_with_center = match i.face {
            JangguFace::궁편 => (viewport.width() / 2) as i32 - near_center_edge_x_pos,
            JangguFace::열편 => near_center_edge_x_pos - (viewport.width() / 2) as i32,
        };
        if distance_with_center <= (janggu_width_min / 2) as i32 {
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

    // draw janggu icon on center
    canvas
        .copy(
            &janggu_texture,
            None,
            Rect::new(
                (viewport.width() - janggu_width) as i32 / 2,
                (viewport.height() - janggu_height) as i32 / 2,
                janggu_width,
                janggu_height,
            ),
        )
        .expect("Failed to draw janggu icon");

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
