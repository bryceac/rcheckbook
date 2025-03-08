use clap::Parser;
use crate::{ database::*, shared::* };
use bcheck::{ Record, Save };
use qif::{Transaction as QIFTransaction, TransactionBuildingError };


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
    }
}

fn record_to_qif(record: &Record) -> Result<QIFTransaction, TransactionBuildingError> {
    let format = qif::DateFormat::MonthDayFullYear;
    let qif_transaction = QIFTransaction::builder()
    .set_date(&record.transaction.date.format(format.chrono_str()).to_string(), &format)
    .set_check_number(record.transaction.check_number.unwrap_or(0))
    .set_vendor(&record.transaction.vendor)
    .set_amount(record.transaction.amount.into_inner())
    .set_category(&record.transaction.category.unwrap_or("".to_owned()))
    .set_memo(&record.transaction.memo)
    .set_status(if record.transaction.is_reconciled { 
        "*" 
    } else {
        ""
    });

    qif_transaction.build();
}

