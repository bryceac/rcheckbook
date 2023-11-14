use std::{ fs, path::Path };
use rusqlite::Connection;
use bcheck::{ Record, Transaction, TransactionType };

pub fn copy_database_if_not_exists(p: &str) {
    if p.starts_with("~") {
        let modified_path = shellexpand::tilde(p).into_owned();

        if !Path::new(&modified_path).exists() {
            let target_path = Path::new(&modified_path);
            let original_path = Path::new("register.db");

            let _ = fs::create_dir_all(&target_path.parent().unwrap());

            let _ = fs::copy(original_path, target_path);
        } else {}
    } else {
        if let Ok(real_path) = fs::canonicalize(p) {

            if !real_path.exists() {
                let original_path = Path::new("register.db");

                let _ = fs::create_dir_all(real_path.as_path().parent().unwrap());

                let _ = fs::copy(original_path, real_path);
            } else {}
        }
    }
}

pub fn load_records_from_db(p: &str) -> Vec<Record> {
    let mut stored_records: Vec<Record> = vec![];

    match Connection::open(p) {
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

                    let memo: String = if let Ok(m) = row.get(4) {
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

    if let Ok(db) = Connection::open(p) {
        if let Ok(mut statement) = db.prepare("SELECT category FROM categories") {
            let category_query = statement.query_map([], |row| {
                let name: String = row.get_unwrap(0);
                Ok(name)
            }).unwrap();

            for category in category_query {
                stored_categories.push(category.unwrap());
            }
        }
    }

    stored_categories
}

pub fn retrieve_balance_for_record(p: &str, r: Record) -> f64 {
    let mut balance = 0.0;
    if let Ok(db) = Connection::open(p) {
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
        if let Ok(db) = Connection::open(p) {
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
        if let Ok(db) = Connection::open(p) {
            let insert_query = format!("INSERT INTO categories(category) VALUES (?1)");

            if let Ok(mut statement) = db.prepare(&insert_query) {
                if let Err(error) = statement.execute([c]) {
                    println!("{}", error);
                }
            }
        }
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

    let date_string = format!("{}", r.transaction.date.format("%Y-%m-%d"));

    let check_number = if let Some(check) = r.transaction.check_number {
        format!("{}", check)
    } else {
        String::default()
    };

    let amount = match r.transaction.transaction_type {
        TransactionType::Withdrawal => format!("{:.2}", r.transaction.amount*-1.0),
        TransactionType::Deposit => format!("{:.2}", r.transaction.amount)
    };

    let reconciled = if r.transaction.is_reconciled {
        format!("{}", 1)
    } else {
        format!("{}", 0)
    };

    let category_id_string = if let Some(id) = category_id {
        format!("{}", id)
    } else {
        String::default()
    };

    if let Ok(db) = Connection::open(p) {
        let insert_statement = format!("INSERT INTO trades VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)");

        if let Ok(mut statement) = db.prepare(&insert_statement) {

            if let Err(error) = statement.execute([r.id.clone(), date_string, check_number, r.transaction.vendor.clone(), r.transaction.memo.clone(), amount, category_id_string, reconciled]) {
                println!("{}", error);
            }
        }
    }
}