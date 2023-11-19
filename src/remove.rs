use clap::Parser;
use crate::database::*;

#[derive(Parser)]
pub struct Remove {
    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,

    #[clap(long, short)]
    pub id: String
}

impl Remove {
    pub fn run(&self) {
        self.remove_record(&self.file_path);
    }

    fn remove_record(&self, p: &str) {
        delete_record_from_db(p, &self.id);
    }
}