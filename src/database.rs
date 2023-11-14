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

                    let amount = if let Ok(a) = row.get(7) {
                        a
                    } else {
                        0.0
                    };

                    let transaction_type = if amount > 0.0 {
                        TransactionType::Deposit
                    } else {
                        TransactionType::Withdrawal
                    };

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
            let _ = Connection::close(db);
        },
        _ => {}
    }

    stored_records
}

pub fn retrieve_balance_for_record(p: &str, r: Record) -> f64 {
    let mut balance = 0.0;
    if let Ok(db) = Connection::open(p) {
        let balance_query_string = format!("SELECT balance FROM ledger WHERE id = {}", r.id);
        if let Ok(mut statement) = db.prepare(&balance_query_string) {
            let balance_query = statement.query_map([], |row| {
                let value: f64 = row.get_unwrap(0);
                Ok(value)
            }).unwrap();

            for amount in balance_query {
                balance = amount.unwrap();
            }
        }
    } else {}

    balance
}