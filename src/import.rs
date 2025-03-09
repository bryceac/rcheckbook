use clap::Parser;
use crate::{ database::*, shared::* };
use bcheck::{ Record, Transaction, TransactionType };
use qif::{ DateFormat, QIF, Transaction as QIFTransaction, TransactionStatus, Type as QIFType };


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
        if let Some(ref category) = transaction.category.clone() {
            Some(category.as_str())
        } else {
            None
        }, 
        &transaction.vendor, 
        &transaction.memo, 
        transaction.amount.abs(), 
        if transaction.amount <= 0.0 {
            TransactionType::Withdrawal
        } else {
            TransactionType::Deposit
        }, 
        if let Some(status) = transaction.status.clone() {
            status == TransactionStatus::Reconciled
        } else {
            false
        })
}

fn records_from_section(qif: QIF, section: QIFType) -> Vec<Record> {
    match section {
        QIFType::Bank => if let Some(bank) = qif.bank {
            bank.transactions.into_iter()
            .map(|t| qif_transaction_to_transaction(&t))
            .filter(|t| t.is_ok())
            .map(|t| Record::from("", t.unwrap()))
            .collect()
        } else {
            vec![]
        },
        QIFType::Cash => if let Some(cash) = qif.cash {
            cash.transactions.into_iter()
            .map(|t| qif_transaction_to_transaction(&t))
            .filter(|t| t.is_ok())
            .map(|t| Record::from("", t.unwrap()))
            .collect()
        } else {
            vec![]
        }
        QIFType::CreditCard => if let Some(credit_card) = qif.credit_card {
            credit_card.transactions.into_iter()
            .map(|t| qif_transaction_to_transaction(&t))
            .filter(|t| t.is_ok())
            .map(|t| Record::from("", t.unwrap()))
            .collect()
        } else {
            vec![]
        },
        QIFType::Liability => if let Some(liability) = qif.liability {
            liability.transactions.into_iter()
            .map(|t| qif_transaction_to_transaction(&t))
            .filter(|t| t.is_ok())
            .map(|t| Record::from("", t.unwrap()))
            .collect()
        } else {
            vec![]
        },
        QIFType::Asset => if let Some(asset) = qif.asset {
            asset.transactions.into_iter()
            .map(|t| qif_transaction_to_transaction(&t))
            .filter(|t| t.is_ok())
            .map(|t| Record::from("", t.unwrap()))
            .collect()
        } else {
            vec![]
        },
    }
}

