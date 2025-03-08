use clap::Parser;
use crate::{ database::*, shared::* };
use bcheck::{ Save, Transaction };
use qif::Transaction as QIFTransaction;


#[derive(Parser)]
pub struct Export {

    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,

    #[clap(long, short)]
    pub output_file: String
}

impl Export {
    pub fn run(&self) {
        self.export();
    }

    fn export(&self) {
        let destination_path = real_path(&self.output_file);
        let records = load_records_from_db(&self.file_path);

        if destination_path.ends_with(".bcheck") {
            if let Err(error) = records.save(&destination_path) {
                println!("{}", error);
            }
        } else {
            if let Err(error) = records.save_tsv(&destination_path) {
                println!("{}", error);
            }
        }
    
    
    fn records_to_qif(records: &Vec<Record>) {

    }
}

fn transaction_to_qif(transaction: &Transaction)

