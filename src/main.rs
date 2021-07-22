mod rcheckbook;
mod records;

use rcheckbook::RCheckbook;
use records::Records;
use std::{ fs };
use clap::Clap;

fn main() {
    let checkbook: RCheckbook = RCheckbook::parse();

    if checkbook.file_path.starts_with("~") {
        let modified_path = shellexpand::tilde(&checkbook.file_path).into_owned();

        load_from(&modified_path)
    } else {
        if let Ok(real_path) = fs::canonicalize(checkbook.file_path) {
            if let Some(file_path) = real_path.to_str() {
                load_from(file_path)
            }
        }
    }
}

fn load_from(p: &str) {
    match Records::from_file(p) {
        Ok(mut records) => {
            records.display()
        },
        Err(error) => {
            println!("{}", error)
        }
    }
}