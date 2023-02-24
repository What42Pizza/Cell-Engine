use crate::prelude::*;
use std::io::Error as IoError;
use sdl2::render::{TextureValueError, UpdateTextureError};
use ab_glyph::InvalidFont;



#[derive(Debug)]
pub enum ProgramError {

    GlyphRenderError {
        glyph: Glyph,
    },

    String (String),
    TextureValueError (TextureValueError),
    UpdateTextureError (UpdateTextureError),
    InvalidFont (InvalidFont),
    IoError (IoError),

}



impl From<String> for ProgramError {
    fn from(input: String) -> Self {
        Self::String(input)
    }
}

impl From<TextureValueError> for ProgramError {
    fn from(input: TextureValueError) -> Self {
        Self::TextureValueError(input)
    }
}

impl From<UpdateTextureError> for ProgramError {
    fn from(input: UpdateTextureError) -> Self {
        Self::UpdateTextureError(input)
    }
}

impl From<InvalidFont> for ProgramError {
    fn from(input: InvalidFont) -> Self {
        Self::InvalidFont(input)
    }
}

impl From<IoError> for ProgramError {
    fn from(input: IoError) -> Self {
        Self::IoError(input)
    }
}
