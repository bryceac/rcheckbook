use clap::Clap;
use std::fs;
use crate::records::Records;

#[derive(Clap)]
pub struct List {
    pub file_path: String,

    #[clap(long)]
    pub show_ids: bool
}

impl List {
    pub fn run(&self) {
        if self.file_path.starts_with("~") {
            let modified_path = shellexpand::tilde(&self.file_path).into_owned();
    
            self.load_from(&modified_path)
        } else {
            if let Ok(real_path) = fs::canonicalize(self.file_path.clone()) {
                if let Some(file_path) = real_path.to_str() {
                    self.load_from(file_path)
                }
            }
        }
    }

    fn load_from(&self, p: &str) {
        match Records::from_file(p) {
            Ok(mut records) => {
                records.display()
            },
            Err(error) => {
                println!("{}", error)
            }
        }
    }
}

