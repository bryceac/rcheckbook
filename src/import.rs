use std::path::Path;

use clap::Parser;
use crate::{ database::*, shared::* };
use bcheck::{ Record, Transaction, TransactionType };
use qif::{ DateFormat, QIF, Transaction as QIFTransaction, TransactionStatus, Type as QIFType };
use spsheet::{ Value, ods, Sheet };


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
            ref p if p.ends_with("qif") => if let Ok(qif) = QIF::load_from_file(p, &DateFormat::MonthDayFullYear) {
                records_from_section(qif, QIFType::Bank)
            } else {
                vec![]
            }
            ref p if p.ends_with(".tsv") => {
                if let Ok(retrieved_records) = Record::from_tsv_file(&source_path) {
                    retrieved_records
                } else {
                    vec![]
                }
            },
            ref p if p.ends_with(".ods") => records_from_ods(p),
            _ => vec![]
        };

        add_records_to_db(&self.file_path, &records)
    }
}

fn qif_transaction_to_transaction(transaction: &QIFTransaction) -> Result<Transaction, String> {
    Transaction::from(
        Some(&transaction.date.format(&DateFormat::FullYearMonthDay.chrono_str()).to_string()), 
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

fn record_from_row(row_index: usize, sheet: &Sheet) -> Option<Record> {
    let id = if let Some(cell) = sheet.get_cell(row_index, 0) {
        if let Value::Str(id) = cell.get_value() {
            Some(id.to_owned())
        } else {
            None
        }
    } else {
        None
    };

    let date = if let Some(cell) = sheet.get_cell(row_index, 1) {
        if let Value::Date(date) = cell.get_value() {
            Some(date)
        } else {
            None
        }
    } else {
        None
    };

    let check_number = if let Some(cell) = sheet.get_cell(row_index, 2) {
        if let Value::Str(check_number) = cell.get_value() {
            if let Ok(number) = check_number.parse::<u32>() {
                Some(number)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let is_reconciled = if let Some(cell) = sheet.get_cell(row_index, 3) {
        if let Value::Str(is_reconciled) = cell.get_value() {
            is_reconciled.to_uppercase() == "Y"
        } else {
            false
        }
    } else {
        false
    };

    let category = if let Some(cell) = sheet.get_cell(row_index, 4) {
        if let Value::Str(category) = cell.get_value() {
            if category.is_empty() {
                None
            } else {
                Some(category.as_str())
            }
        } else {
            None
        }
    } else {
        None
    };

    let vendor = if let Some(cell) = sheet.get_cell(row_index, 5) {
        if let Value::Str(vendor) = cell.get_value() {
            if vendor.is_empty() {
                None
            } else {
                Some(vendor.as_str())
            }
        } else {
            None
        }
    } else {
        None
    };

    let memo = if let Some(cell) = sheet.get_cell(row_index, 6) {
        if let Value::Str(memo) = cell.get_value() {
            Some(memo.as_str())
        } else {
            None
        }
    } else {
        None
    };

    let mut transaction_type: Option<TransactionType> = None;

    let amount = if let Some(cell) = sheet.get_cell(row_index, 7) {
        if let Value::Float(amount) = cell.get_value() {
            transaction_type = Some(TransactionType::Deposit);
            amount.to_owned()
        } else {
            0.0
        }
    } else if let Some(cell) = sheet.get_cell(row_index, 8) {
        if let Value::Float(amount) = cell.get_value() {
            transaction_type = Some(TransactionType::Withdrawal);
            amount.to_owned()
        } else {
            0.0
        }
    } else {
        0.0
    };

    if let None = date {
        None
    } else if let None = vendor {
        None
    } else {
        let date_string = format!("{}", date.unwrap().format("%Y-%m-%d"));
        let transaction = Transaction::from(
            Some(&date_string), 
            check_number, 
            category, 
            vendor.unwrap(), 
            memo.unwrap_or(""), 
            amount, 
            transaction_type.unwrap_or(TransactionType::Withdrawal), 
            is_reconciled);

        Some(Record::from(id.as_deref().unwrap_or(""), transaction.unwrap()))
    }
}

fn records_from_ods(p: &str) -> Vec<Record> {
    let mut records: Vec<Record> = vec![];

    if let Ok(book) = ods::read(&Path::new(p)) {
        let sheet = book.get_sheet(0);

        let number_of_rows = sheet.get_rows().len();

        for row_index in 0..number_of_rows {
            if let Some(record) = record_from_row(row_index, sheet) {
                records.push(record);
            }
        }
    }

    records
}

