use clap::Parser;
use crate::records::Records;
use crate::{ database::* shared::* };


#[derive(Parser)]
pub struct Import {

    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,

    pub origin_file: String
}

impl Import {
    pub fn run(&self) {
        copy_database_if_not_exists(&self.file_path);
    }

    fn import(&self) {
        let source_path = real_path(&self.origin_file);
    }
}

