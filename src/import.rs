use clap::Parser;
use crate::{ database::*, shared::*, errors::ImportError };
use bcheck::{ Record, Transaction, TransactionType };
use qif::{ DateFormat, QIF, Transaction as QIFTransaction, TransactionStatus, Type as QIFType };
use calamine::{ Data, open_workbook, Ods, Xlsx, Reader };


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

fn record_from_ods_row(row: &[Data]) -> Result<Record, ImportError> {
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
                7 => if let calamine::Data::String(record_deposit) = data {
                    if let Ok(credit_amount) = record_withdrawal.parse::<f64>() {
                        credit = credit_amount
                    }
                },
                8 => if let calamine::Data::String(record_withdrawal) = data {
                    if let Ok(withdrawal_amount) = record_withdrawal.parse::<f64>() {
                        withdrawal = withdrawal_amount
                    }
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

fn records_from_ods(p: &str) -> Vec<Record> {
    let mut records = vec![];
    let mut workbook: Ods<_> = open_workbook(p).expect("Could not read workbook");
    let range = workbook.worksheet_range_at(0).unwrap().expect("Could not read sheet");

    let number_of_rows = range.rows().count()-1;

    for (row_index, row) in range.rows().enumerate() {
        if row_index == 0 {
            continue;
        } else {
            println!("attempting to import transaction {} of {} transactions", row_index, number_of_rows);
            match record_from_ods_row(row) {
                Ok(record) => records.push(record),
                Err(error) => println!("{}", error)
            }
        }
    }

    records
}


