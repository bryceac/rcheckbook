use clap::Parser;
use crate::{ database::*, shared::* };
use bcheck::{ Record, Transaction, TransactionType };
use qif::{ DateFormat, Transaction as QIFTransaction, TransactionStatus };


#[derive(Parser)]
pub struct Import {

    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,

    #[clap(long, short)]
    pub input_file: String
}

impl Import {
    pub fn run(&self) {
        copy_database_if_not_exists(&self.file_path);
        self.import();
    }

    fn import(&self) {
        let source_path = real_path(&self.input_file);
        let records = match source_path {
            ref p if p.ends_with(".bcheck") => {
                if let Ok(retrieved_records) = Record::from_file(&source_path) {
                    retrieved_records
                } else {
                    vec![]
                }
            },
            ref p if p.ends_with(".tsv") => {
                if let Ok(retrieved_records) = Record::from_tsv_file(&source_path) {
                    retrieved_records
                } else {
                    vec![]
                }
            },
            _ => vec![]
        };

        add_records_to_db(&self.file_path, &records)
    }
}

fn qif_transaction_to_transaction(transaction: &QIFTransaction) -> Result<Transaction, String> {
    Transaction::from(
        Some(&transaction.date.format(&DateFormat::MonthDayFullYear.chrono_str()).to_string()), 
        transaction.check_number, 
        transaction.category, 
        &transaction.vendor, 
        &transaction.memo, 
        abs(transaction.amount), 
        if transaction.amount <= 0 {
            TransactionType::Withdrawal
        } else {
            TransactionType::Deposit
        }, 
        if let TransactionStatus::Reconciled = transaction.status {
            true
        } else {
            false
        })
}

