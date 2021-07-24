use bcheck::{ Record, Save };
use clap::Clap;
use crate::records::Records;
use std::fs;

#[derive(Clap)]
pub struct Remove {
    pub file_path: String,

    #[clap(long, short)]
    pub id: String
}

impl Remove {
    pub fn run(&self) -> Result<(), String> {
        let mut stored_records = Records::from_file(&self.file_path)?;

        stored_records.remove(self.id.clone());

        match stored_records.sorted_records().save(&self.file_path) {
            Ok(()) => Ok(()),
            Err(error) => Err(error.to_string())
        }
    }
}