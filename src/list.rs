use clap::Parser;
use bcheck::Record;
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

pub fn display(store: &Records, db: &str) {
    for record in store.sorted_records() {
        let balance = store.balance_for_record(db, &record);
        println!("{}\t{:.2}", record, balance);
    }
}

pub fn filtered_display(store: &Records, category: Option<String>, db: &str) {
    if let Some(category) = category {
        let filtered_records: Vec<Record> = store.sorted_records().into_iter().filter(|record| record.transaction.category.clone().unwrap_or("Uncategorized".to_string()).to_lowercase() == category.to_lowercase()).collect();

        for record in filtered_records {
            let balance = store.balance_for_record(db, &record);
            println!("{}\t{:.2}", record, balance);
        }
    } else {
        println!("{}", "Nothing matched the specified criteria")
    }
}

