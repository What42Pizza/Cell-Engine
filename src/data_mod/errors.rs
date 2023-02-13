use crate::prelude::*;



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



#[derive(Debug)]
pub enum RawProgramError {

    String (String),

}
