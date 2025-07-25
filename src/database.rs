use std::{ fs, path::{ Path, PathBuf } };

#[cfg(windows)]
use std::env;

use rusqlite::{ Connection, params };
use bcheck::{ Record, Transaction, TransactionType };
use crate::shared::*;

pub fn copy_database_if_not_exists(p: &str) {
    let target = real_path(p);
    let destination_path = Path::new(&target);

    #[cfg(windows)]
    let original_path: PathBuf = if let Ok(path) = env::current_exe() {
        if let Some(db_directory) = path.parent() {
            db_directory.join("register.db")
        } else {
            Path::new("register.db").to_path_buf()  
        }
    } else {
        Path::new("register.db").to_path_buf()
    };

    #[cfg(unix)]
    let original_path: PathBuf = Path::new(&real_path("/var/db/rcheckbook/register.db")).to_path_buf();
    

    if !destination_path.exists() {
        let _ = fs::create_dir_all(destination_path.parent().unwrap());
        let _ = fs::copy(original_path, destination_path);
    }
}

pub fn load_records_from_db(p: &str) -> Vec<Record> {
    let mut stored_records: Vec<Record> = vec![];

    match Connection::open(&real_path(p)) {
        Ok(db) => {
            if let Ok(mut statement) = db.prepare("SELECT * from ledger") {
                let record_query = statement.query_map([], |row| {
                    let id: String = row.get_unwrap(0);
                    let date_string: String = row.get_unwrap(1);
                    let check_number: Option<u32> = if let Ok(num) = row.get(2) {
                        Some(num)
                    } else {
                        None
                    };
                    let category: Option<String> = if let Ok(c) = row.get(6) {
                        Some(c)
                    } else {
                        None
                    };
                    let vendor: String = if let Ok(v) = row.get(4) {
                        v
                    } else {
                        String::default()
                    };

                    let memo: String = if let Ok(m) = row.get(5) {
                        m
                    } else {
                        String::default()
                    };

                    let mut amount: f64 = if let Ok(a) = row.get(7) {
                        a
                    } else {
                        0.0
                    };

                    let transaction_type = if amount > 0.0 {
                        TransactionType::Deposit
                    } else {
                        TransactionType::Withdrawal
                    };

                    amount = amount.abs();

                    let is_reconciled = if let Ok(r) = row.get(3) {
                        r
                    } else {
                        String::from("N")
                    };

                    Ok(Record::from(&id, 
                    Transaction::from(Some(&date_string),
                    check_number, 
                    category.as_deref(), 
                    &vendor, 
                    &memo, 
                    amount, 
                    transaction_type, 
                    if is_reconciled == "Y" { true } else { false }).unwrap()))
                }).unwrap();
                
                for row in record_query {
                    if let Ok(record) = row {
                        stored_records.push(record);
                    }
                }
            }
        },
        _ => {}
    }

    stored_records
}

pub fn load_categories_from_db(p: &str) -> Vec<String> {
    let mut stored_categories: Vec<String> = vec![];

    if let Ok(db) = Connection::open(&real_path(p)) {
        if let Ok(mut statement) = db.prepare("SELECT category FROM categories") {
            let category_query = statement.query_map([], |row| {
                let name: String = row.get_unwrap(0);
                Ok(name)
            }).unwrap();

            for category in category_query {
                stored_categories.push(category.unwrap());
            }
        }
    } else {}
    stored_categories
}

pub fn retrieve_balance_for_record(p: &str, r: Record) -> f64 {
    let mut balance = 0.0;
    if let Ok(db) = Connection::open(&real_path(p)){
        let balance_query_string = format!("SELECT balance FROM ledger WHERE id = '{}'", r.id);
        balance = db.query_row(&balance_query_string, [], |row| row.get(0)).unwrap();
    } else {}

    balance
}

fn category_exists_in_db(p: &str, c: &str) -> bool {
    let categories: Vec<String> = load_categories_from_db(p).iter().map(|e| e.to_lowercase()).collect();
    categories.contains(&c.to_lowercase())
}

fn category_id(p: &str, c: &str) -> Option<i32> {
    let mut category_id: Option<i32> = None;
    if category_exists_in_db(p, c) {
        if let Ok(db) = Connection::open(&real_path(p)) {
            let category_sql = format!("SELECT id FROM categories WHERE category = '{}' COLLATE NOCASE", c);
            category_id = if let Ok(value) = db.query_row(&category_sql, [], |row| row.get(0)) {
                Some(value)
            } else {
                None
            }
        }
        category_id
    } else {
        category_id
    }
}

pub fn add_category_to_db(p: &str, c: &str) {
    if !category_exists_in_db(p, c) {
        if let Ok(db) = Connection::open(&real_path(p)) {
            let insert_query = format!("INSERT INTO categories(category) VALUES (?1)");

            if let Ok(mut statement) = db.prepare(&insert_query) {
                if let Err(error) = statement.execute([c]) {
                    println!("{}", error);
                }
            }
        } else {}
    }
}

pub fn add_record_to_db(p: &str, r: &Record) {
    let category_id = if let Some(category) = &r.transaction.category {
        if let Some(id) = category_id(p, category) {
            Some(id)
        } else {
            add_category_to_db(p, category);
            category_id(p, &category)
        }
    } else {
        None
    };

    if let Ok(db) = Connection::open(&real_path(p)) {
        let insert_statement = format!("INSERT INTO trades VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)");

        if let Ok(mut statement) = db.prepare(&insert_statement) {
            if let Err(error) = statement.execute(params![r.id.to_uppercase(),format!("{}", r.transaction.date.format("%Y-%m-%d")), r.transaction.check_number, r.transaction.vendor, r.transaction.memo, if let TransactionType::Deposit = r.transaction.transaction_type { r.transaction.amount.into_inner() } else { r.transaction.amount.into_inner()*-1.0 }, category_id, r.transaction.is_reconciled]) {
                println!("{}", error);
            }
        }
    } else {}
}

pub fn retrieve_record_with_id_from_db(p: &str, i: &str) -> Option<Record> {
    let records = load_records_from_db(p);

    let index_of_record = records.iter().position(|record| record.id.to_lowercase() == i.to_lowercase());

    match index_of_record {
        Some(index) => Some(records[index].clone()),
        _ => None
    }
}

pub fn update_record_in_db(p: &str, r: &Record) {
    let category_id = if let Some(category) = &r.transaction.category {
        if let Some(id) = category_id(p, category) {
            Some(id)
        } else {
            add_category_to_db(p, category);
            category_id(p, &category)
        }
    } else {
        None
    };

    if let Some(record) = retrieve_record_with_id_from_db(p, &r.id) {
        if let Ok(db) = Connection::open(&real_path(p)) {
            if r.transaction != record.transaction {
                let update_statement = format!("UPDATE trades SET date = (?1), check_number = (?2), vendor = (?3), memo = (?4), amount = (?5), category = (?6), reconciled = (?7) WHERE id = (?8)");

                if let Ok(mut statement) = db.prepare(&update_statement) {
                    if let Err(error) = statement.execute(params![format!("{}", r.transaction.date.format("%Y-%m-%d")), r.transaction.check_number, r.transaction.vendor, r.transaction.memo, if let TransactionType::Deposit = r.transaction.transaction_type { r.transaction.amount.into_inner() } else { r.transaction.amount.into_inner()*-1.0 }, category_id, r.transaction.is_reconciled, record.id]) {
                        println!("{}", error);
                    }
                }
            }
        }
    }
}

pub fn delete_record_from_db(p: &str, i: &str) {
    if let Some(_) = retrieve_record_with_id_from_db(p, i) {
        if let Ok(db) = Connection::open(&real_path(p)) {
            let delete_statement = format!("DELETE FROM trades WHERE id = (?)");

            if let Ok(mut statement) = db.prepare(&delete_statement) {
                if let Err(error) = statement.execute(params![]) {
                    println!("{}", error);
                }
            }
        }
    }
}

pub fn update_or_add_record(p: &str, r: &Record) {
    if let Some(_) = retrieve_record_with_id_from_db(p, &r.id) {
        update_record_in_db(p, r);
    } else {
        add_record_to_db(p, r);
    }
}

pub fn add_records_to_db(p: &str, r: &Vec<Record>) {
    for record in r {
        update_or_add_record(p, record);
    }
}