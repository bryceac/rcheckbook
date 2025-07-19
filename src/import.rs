use std::path::Path;

use clap::Parser;
use crate::{ database::*, shared::* };
use bcheck::{ Record, Transaction, TransactionType, is_proper_date_format };
use qif::{ DateFormat, QIF, Transaction as QIFTransaction, TransactionStatus, Type as QIFType };
// use spsheet::{ Value, ods, Sheet, xlsx };
use calamine::{ Data, open_workbook, Xlsx, Ods, Reader };


#[derive(Parser)]
#[clap(version = "0.4", author = "Bryce Campbell")]
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

fn records_from_xlsx(p: &str) -> Vec<Record> {
    let mut records = vec![];
    let mut workbook: Xlsx<_> = open_workbook(p).expect("Could not read workbook");
    let range = workbook.worksheet_range_at(0).unwrap().expect("Could not read sheet");

    for row in range.rows() {
        if let Some(record) = record_from_row(row) {
            records.push(record)
        }
    }

    records
}

fn record_from_row(row: &[Data]) -> Option<Record> {
    let mut record: Option<Record> = None;
    let mut id = "";
    let mut date = "";
    let mut check_number = 0;
    let mut is_reconciled = false;
    let mut category = "";
    let mut vendor = "";
    let mut memo = "";
    let mut credit = 0.0;
    let mut withdrawal = 0.0;
    let transaction_type = if credit > 0.0 {
        TransactionType::Deposit
    } else {
        TransactionType::Withdrawal
    };

    let amount = if credit > 0.0 {
        credit
    } else {
        withdrawal
    };

    for (column_index, data) in row.iter().enumerate() {
        match column_index {
            0 => if let calamine::Data::String(record_id) = data {
                id = record_id;
            },
            1 => if let calamine::Data::String(record_date) = data {
                date = record_date;
            },
            2 => if let calamine::Data::Int(record_check_number) = data {
                check_number = record_check_number;
            },
            3 => if let calamine::Data::Bool(record_reconciled) = data {

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
                credit = record_deposit
            },
            8 => if let calamine::Data::Float(record_withdrawal) = data {
                withdrawal = record_withdrawal;
            },
            _ => ()
        }
    }

    record
}

/* fn record_from_row(row_index: usize, sheet: &Sheet) -> Option<Record> {
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
        if let Value::Str(date) = cell.get_value() {
            Some(date.as_str())
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

    let credit = if let Some(cell) = sheet.get_cell(row_index, 7) {
        if let Value::Float(amount) = cell.get_value() {
            Some(amount.to_owned())
        } else {
            None
        }
    } else {
        None
    };

    let withdrawal = if let Some(cell) = sheet.get_cell(row_index, 8) {
        if let Value::Float(amount) = cell.get_value() {
            Some(amount.to_owned())
        } else {
            None
        }
    } else {
        None
    };

    let transaction_type: TransactionType = if let Some(_) = credit {
        TransactionType::Deposit
    } else {
        TransactionType::Withdrawal
    };

    let amount =  if let Some(credit) = credit {
        credit
    } else if let Some(withdrawal) = withdrawal {
        withdrawal
    } else {
        0.0
    };

    if let None = date {
        None
    } else if let None = vendor {
        None
    } else if !is_proper_date_format(date.unwrap()) {
        None
    } else {
        let transaction = Transaction::from(
            date, 
            check_number, 
            category, 
            vendor.unwrap(), 
            memo.unwrap_or(""), 
            amount, 
            transaction_type, 
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

fn records_from_xlsx(p: &str) -> Vec<Record> {
    let mut records: Vec<Record> = vec![];

    if let Ok(book) = xlsx::read(&Path::new(p)) {
        let sheet = book.get_sheet(0);

        let number_of_rows = sheet.get_rows().len();

        for row_index in 0..number_of_rows {
            if let Some(record) = record_from_row(row_index, sheet) {
                records.push(record);
            }
        }
    }

    records
} */

