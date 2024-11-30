use clap::Parser;
use crate::records::Records;
use crate::database::*;


#[derive(Parser)]
pub struct List {

    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,

    #[clap(long, short)]
    pub category: Option<String>
}

impl List {
    pub fn run(&self) {
        copy_database_if_not_exists(&self.file_path);
        let mut record_store = Records::from(load_records_from_db(&self.file_path));

        if let Some(_) = self.category {
            record_store.filtered_display(self.category.clone(), &self.file_path)
        } else {
            record_store.display(&self.file_path);
        }
        
    }
}

