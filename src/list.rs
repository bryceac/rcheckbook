use clap::Parser;
use crate::records::Records;
use bcheck::{ Record, Transaction };
use crate::database::copy_database_if_not_exists;
use rusqlite::Connection;

#[derive(Parser)]
pub struct List {

    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,
}

impl List {
    pub fn run(&self) {
        /* if self.file_path.starts_with("~") {
            let modified_path = shellexpand::tilde(&self.file_path).into_owned();
    
            self.load_from(&modified_path)
        } else {
            if let Ok(real_path) = fs::canonicalize(self.file_path.clone()) {
                if let Some(file_path) = real_path.to_str() {
                    self.load_from(file_path)
                }
            }
        } */
        copy_database_if_not_exists(&self.file_path);
        self.load_from_db(&self.file_path);
    }

    fn load_from_db(&self, p: &str) {
        /* match Records::from_file(p) {
            Ok(mut records) => {
                records.display()
            },
            Err(error) => {
                println!("{}", error)
            }
        } */

        match Connection::open(p) {
            Ok(db) => {
                if let Ok(statement) = db.prepare("SELECT * from ledger") {
                    let record_query = statement.query_map([], |row| Record::from(row.get(0).unwrap(), Transaction::from(row.get(1), row.get(2), row.get(3), row.get(4), row.get(5), row.get(6), row.get(7))));
                }
                let _ = Connection::close(db);
            },
            _ => println!("Could not connect")
        }
    }
}

