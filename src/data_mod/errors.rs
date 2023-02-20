use crate::prelude::*;
use sdl2::{ttf::FontError, render::TextureValueError};



#[derive(Debug)]
pub struct ProgramError {
    pub raw: RawProgramError,
}



impl From<String> for ProgramError {
    fn from(input: String) -> ProgramError {
        Self {
            raw: RawProgramError::String(input),
        }
    }
}

impl From<FontError> for ProgramError {
    fn from(input: FontError) -> ProgramError {
        Self {
            raw: RawProgramError::FontError(input),
        }
    }
}

impl From<TextureValueError> for ProgramError {
    fn from(input: TextureValueError) -> ProgramError {
        Self {
            raw: RawProgramError::TextureValueError(input),
        }
    }
}



#[derive(Debug)]
pub enum RawProgramError {

    String (String),
    FontError (FontError),
    TextureValueError (TextureValueError),

}
