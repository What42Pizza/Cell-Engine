use crate::prelude::*;
use std::{fs::File, io::Write};



pub struct Logger {
    pub output: File,
}

impl Logger {

    pub fn new (output_path: &Path) -> Result<Self, std::io::Error> {
        //if output_path.exists() {fs::remove_file(&output_path)?;}
        let output = File::create(output_path)?;
        Ok(Self {output})
    }

    pub fn log (&mut self, input: &[u8]) {
        let _ = self.output.write_all(input);
        let _ = self.output.write(&[b'\n']);
    }

    pub fn flush (&mut self) -> Result<(), std::io::Error> {
        self.output.flush()
    }

}
