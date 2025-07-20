use std::path::Path;

use clap::Parser;
use crate::{ database::*, shared::*, errors::ImportError };
use bcheck::{ Record, Transaction, TransactionType };
use qif::{ DateFormat, QIF, Transaction as QIFTransaction, TransactionStatus, Type as QIFType };
use calamine::{ Data, open_workbook, Ods, Xlsx, Reader };
// use spsheet::{ Value, ods, Sheet };


#[derive(Parser)]
#[clap(version = "0.4.4", author = "Bryce Campbell")]
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
            ref p if p.ends_with(".xlsx") => records_from_xlsx(p),
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

fn record_from_xlsx_row(row: &[Data]) -> Result<Record, ImportError> {
    let mut id = "";
    let mut date = "";
    let mut check_number = 0;
    let mut is_reconciled = false;
    let mut category = "";
    let mut vendor = "";
    let mut memo = "";
    let mut credit = 0.0;
    let mut withdrawal = 0.0;

    for (column_index, data) in row.iter().enumerate() {
        if let calamine::Data::Empty = data { 
            continue;
        } else {
            match column_index {
                0 => if let calamine::Data::String(record_id) = data {
                    id = record_id;
                },
                1 => if let calamine::Data::String(record_date) = data {
                    date = record_date;
                },
                2 => if let calamine::Data::String(record_check_number) = data {
                    if !record_check_number.is_empty() {
                        check_number = record_check_number.parse::<u32>().expect("value must be a number 0 or greater")
                    }
                },
                3 => if let calamine::Data::String(record_reconciled) = data {
                    is_reconciled = record_reconciled.to_uppercase() == "Y"
                },
                4 => if let calamine::Data::String(record_category) = data {
                    category = record_category;
                },
                5 => if let calamine::Data::String(record_vendor) = data {
                    vendor = record_vendor;
                },
                6 => if let calamine::Data::String(record_memo) = data {
                    memo = record_memo;
                },
                7 => if let calamine::Data::Float(record_deposit) = data {
                    credit = record_deposit.to_owned()
                },
                8 => if let calamine::Data::Float(record_withdrawal) = data {
                    withdrawal = record_withdrawal.to_owned()
                },
                _ => ()
            };
        }
    }

    if credit > 0.0 && withdrawal > 0.0 {
        return Err(ImportError::TransactionTypeParsingError);
    }

    let transaction_type = if credit > 0.0 {
        TransactionType::Deposit
    } else {
        TransactionType::Withdrawal
    };

    let amount = if let TransactionType::Deposit = transaction_type {
        credit
    } else {
        withdrawal
    };

    if let Ok(transaction) = Transaction::from(Some(date), 
        if check_number > 0 {
            Some(check_number)
        } else {
            None
        }, 
        if category.is_empty() {
            None
        } else {
            Some(category)
        }, 
        vendor,
        memo, 
        amount, 
        transaction_type, 
        is_reconciled) {
            Ok(Record::from(id, transaction))
        } else {
            Err(ImportError::InvalidDateFormat)
        }
}

fn records_from_xlsx(p: &str) -> Vec<Record> {
    let mut records = vec![];
    let mut workbook: Xlsx<_> = open_workbook(p).expect("Could not read workbook");
    let range = workbook.worksheet_range_at(0).unwrap().expect("Could not read sheet");

    let number_of_rows = range.rows().count()-1;

    for (row_index, row) in range.rows().enumerate() {
        if row_index == 0 {
            continue;
        } else {
            println!("attempting to import transaction {} of {} transactions", row_index, number_of_rows);
            match record_from_xlsx_row(row) {
                Ok(record) => records.push(record),
                Err(error) => println!("{}", error)
            }
        }
    }

    records
}

/* fn record_from_ods_row(row_index: usize, sheet: &Sheet) -> Result<Record, ImportError> {
    let mut id = "";
    let mut date = "";
    let mut check_number = 0;
    let mut is_reconciled = false;
    let mut category = "";
    let mut vendor = "";
    let mut memo = "";
    let mut credit = 0.0;
    let mut withdrawal = 0.0;

    if let Some(cell) = sheet.get_cell(row_index, 0) {
        if let Value::Str(record_id) = cell.get_value() {
            id = record_id.as_str()
        }
    };

    if let Some(cell) = sheet.get_cell(row_index, 1) {
        if let Value::Str(record_date) = cell.get_value() {
            date = record_date.as_str()
        }
    };

    if let Some(cell) = sheet.get_cell(row_index, 2) {
        if let Value::Str(record_check_number) = cell.get_value() {
            if let Ok(number) = record_check_number.parse::<u32>() {
                check_number = number
            }
        }
    };

    if let Some(cell) = sheet.get_cell(row_index, 3) {
        if let Value::Str(record_reconciled) = cell.get_value() {
            is_reconciled = record_reconciled.to_uppercase() == "Y"
        }
    };

    if let Some(cell) = sheet.get_cell(row_index, 4) {
        if let Value::Str(record_category) = cell.get_value() {
            category = record_category.as_str()
        }
    };

    if let Some(cell) = sheet.get_cell(row_index, 5) {
        if let Value::Str(record_vendor) = cell.get_value() {
            vendor = record_vendor.as_str()
        }
    };

    if let Some(cell) = sheet.get_cell(row_index, 6) {
        if let Value::Str(record_memo) = cell.get_value() {
            memo = record_memo.as_str()
        }
    };

    if let Some(cell) = sheet.get_cell(row_index, 7) {
        if let Value::Float(amount) = cell.get_value() {
            credit = amount.to_owned()
        }
    };

    if let Some(cell) = sheet.get_cell(row_index, 8) {
        if let Value::Float(amount) = cell.get_value() {
            withdrawal = amount.to_owned()
        }
    };

    if credit > 0.0 && withdrawal > 0.0 {
        return Err(ImportError::TransactionTypeParsingError);
    }

    let transaction_type: TransactionType = if credit > 0.0 {
        TransactionType::Deposit
    } else {
        TransactionType::Withdrawal
    };

    let amount =  if let TransactionType::Deposit = transaction_type {
        credit
    }  else {
        withdrawal
    };

    if let Ok(transaction) = Transaction::from(Some(date), 
        if check_number > 0 {
            Some(check_number)
        } else {
            None
        }, 
        if category.is_empty() {
            None
        } else {
            Some(category)
        }, 
        vendor,
        memo, 
        amount, 
        transaction_type, 
        is_reconciled) {
            Ok(Record::from(id, transaction))
        } else {
            Err(ImportError::InvalidDateFormat)
        }
}

fn records_from_ods(p: &str) -> Vec<Record> {
    let mut records: Vec<Record> = vec![];

    if let Ok(book) = ods::read(&Path::new(p)) {
        let sheet = book.get_sheet(0);

        let number_of_rows = sheet.get_rows().len();

        for row_index in 0..number_of_rows {
            if row_index == 0 {
                continue;
            } else {
                println!("attempting to import transaction {} of {} transactions", row_index, number_of_rows-1);
                match record_from_ods_row(row_index, sheet) {
                    Ok(record) => records.push(record),
                    Err(error) => println!("{}", error)
                }
            }
            
        }
    }

    records
} */


