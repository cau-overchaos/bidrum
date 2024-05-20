use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use crate::constants::DEFAULT_IMG_PATH as IMG_PATH;

pub struct NoteTextures<'a> {
    pub left_stick: Texture<'a>,
    pub right_stick: Texture<'a>,
}
pub struct AccuracyTextures<'a> {
    pub overchaos: Texture<'a>,
    pub perfect: Texture<'a>,
    pub great: Texture<'a>,
    pub good: Texture<'a>,
    pub bad: Texture<'a>,
    pub miss: Texture<'a>,
}
pub struct ChartPlayerUIResources<'a> {
    pub judgement_line_texture: Texture<'a>,
    pub note_textures: NoteTextures<'a>,
    pub accuray_textures: AccuracyTextures<'a>,
    pub janggu_texture: Texture<'a>,
}

fn load_note_textures(
    texture_creator: &TextureCreator<WindowContext>,
) -> Result<NoteTextures, String> {
    Ok(NoteTextures {
        left_stick: texture_creator.load_texture(IMG_PATH.to_owned() + "note/left_stick.png")?,
        right_stick: texture_creator.load_texture(IMG_PATH.to_owned() + "note/right_stick.png")?,
    })
}

fn load_accuracy_textures(
    texture_creator: &TextureCreator<WindowContext>,
) -> Result<AccuracyTextures, String> {
    Ok(AccuracyTextures {
        overchaos: texture_creator
            .load_texture(&(IMG_PATH.to_owned() + "accuracy/overchaos.png"))?,
        perfect: texture_creator.load_texture(IMG_PATH.to_owned() + "accuracy/perfect.png")?,
        great: texture_creator.load_texture(IMG_PATH.to_owned() + "accuracy/great.png")?,
        good: texture_creator.load_texture(IMG_PATH.to_owned() + "accuracy/good.png")?,
        bad: texture_creator.load_texture(IMG_PATH.to_owned() + "accuracy/bad.png")?,
        miss: texture_creator.load_texture(IMG_PATH.to_owned() + "accuracy/miss.png")?,
    })
}

impl ChartPlayerUIResources<'_> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> ChartPlayerUIResources {
        let judgement_line_texture = texture_creator
            .load_texture(IMG_PATH.to_owned() + "play_ui/note_guideline.png")
            .expect("Failed to load note guideline image");
        let note_textures = load_note_textures(texture_creator).unwrap();
        let janggu_texture = texture_creator
            .load_texture(IMG_PATH.to_owned() + "play_ui/janggu.png")
            .expect("Failed to load janggu image");
        let accuray_textures = load_accuracy_textures(texture_creator).unwrap();

        return ChartPlayerUIResources {
            judgement_line_texture: judgement_line_texture,
            note_textures: note_textures,
            janggu_texture: janggu_texture,
            accuray_textures: accuray_textures,
        };
    }
}
