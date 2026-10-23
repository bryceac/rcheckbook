use std::{ fs::{ self, File }, path::{ Path } };

use std::env;

use rusqlite::{ Connection, params };
use bcheck::{ Record, Transaction, TransactionType };
use crate::shared::*;

pub fn create_database_if_not_exists(p: &str) {
    let target = real_path(p);
    let destination_path = Path::new(&target);

    if !destination_path.exists() {
        let _ = fs::create_dir_all(destination_path.parent().unwrap());

        let db = Connection::open(&destination_path).unwrap();

        if let Err(error) = db.execute_batch("BEGIN TRANSACTION;
            DROP TABLE IF EXISTS \"categories\";
            CREATE TABLE \"categories\" (
	            \"id\"	INTEGER,
	            \"category\"	TEXT NOT NULL,
	            PRIMARY KEY(\"id\" AUTOINCREMENT)
            ) STRICT;
            DROP TABLE IF EXISTS \"trades\";
            CREATE TABLE \"trades\" (
	            \"id\"	TEXT,
	            \"date\"	TEXT NOT NULL DEFAULT (DATE('now')),
	            \"check_number\"	INTEGER DEFAULT NULL,
	            \"vendor\"	TEXT DEFAULT '',
	            \"memo\"	TEXT DEFAULT '',
	            \"amount\"	REAL NOT NULL DEFAULT 0.0,
	            \"category\"	INTEGER DEFAULT NULL,
	            \"reconciled\"	INTEGER NOT NULL DEFAULT 0 CHECK(\"reconciled\" IN (0, 1)),
	            PRIMARY KEY("id")
            ) STRICT;
            INSERT INTO \"categories\" VALUES (1,'Utilities'),
            (2,'Gifts'),
            (3,'Groceries'),
            (4,'Dining'),
            (5,'Recreation'),
            (6,'Subscriptions'),
            (7,'Opening Balance');
            DROP VIEW IF EXISTS \"ledger\";
            CREATE VIEW ledger AS
            SELECT t.id AS 'id',
            DATE(t.date) AS 'date', 
            t.check_number, 
            CASE t.reconciled
	            WHEN 1
		            THEN 'Y'
	            ELSE 'N'
            END AS 'reconciled',
            t.vendor, 
            t.memo,
            c.category,
            t.amount,
            SUM(t.amount) OVER (
	            ORDER BY DATE(t.date) ASC
	            ROWS BETWEEN
	            UNBOUNDED PRECEDING
	            AND CURRENT ROW) AS 'balance'
            FROM trades t
            LEFT JOIN categories c 
            ON t.category = c.id 
            ORDER BY DATE(t.date) ASC;
            COMMIT;") {
                println!("{}", error)
        }
    }
}

pub fn load_records_from_db(p: &str) -> Vec<Record> {
    let mut stored_records: Vec<Record> = vec![];
    let db = Connection::open(&real_path(p)).unwrap();

    if let Ok(mut statement) = db.prepare("SELECT * from ledger") {
        let record_query = statement.query_map([], |row| {
            let id: String = row.get_unwrap(0);

            let date: String = row.get_unwrap(1);
            let check_number: u32 = if let Ok(num) = row.get(2) {
                num
            } else {
                0
            };

            let category: String = if let Ok(c) = row.get(6) {
                c
            } else {
                String::default()
            };

            let vendor: String = row.get_unwrap(4);

            let memo: String = row.get_unwrap(5);

            let amount: f64 = if let Ok(a) = row.get(7) {
                a
            } else {
                0.0
            };

            let is_reconciled: String = if let Ok(r) = row.get(3) {
                r
            } else {
                "N".to_string()
            };

            let transaction = Transaction::builder()
                .set_date(&date)
                .set_check_number(check_number)
                .set_category(&category)
                .set_vendor(&vendor)
                .set_memo(&memo)
                .set_amount_and_type(amount)
                .set_is_reconciled(is_reconciled.to_lowercase() == "y")
                .build();

                Ok(Record::from(&id, transaction))
        }).unwrap();
                
        for row in record_query {
            if let Ok(record) = row {
                stored_records.push(record);
            }
        }
    }

    stored_records
}

pub fn load_categories_from_db(p: &str) -> Vec<String> {
    let mut stored_categories: Vec<String> = vec![];
    let db = Connection::open(&real_path(p)).unwrap();

    if let Ok(mut statement) = db.prepare("SELECT category FROM categories") {
        let category_query = statement.query_map([], |row| {
            let name: String = row.get_unwrap(0);
            Ok(name)
        }).unwrap();

        for category in category_query {
            stored_categories.push(category.unwrap());
        }
    }

    stored_categories
}

pub fn retrieve_balance_for_record(p: &str, r: Record) -> f64 {
    let mut balance = 0.0;
    let db = Connection::open(&real_path(p)).unwrap();

    let balance_query_string = format!("SELECT balance FROM ledger WHERE id = '{}'", r.id);
    balance = db.query_row(&balance_query_string, [], |row| row.get(0)).unwrap();

    balance
}

fn category_exists_in_db(p: &str, c: &str) -> bool {
    let categories: Vec<String> = load_categories_from_db(p).iter().map(|e| e.to_lowercase()).collect();
    categories.contains(&c.to_lowercase())
}

fn category_id(p: &str, c: &str) -> Option<i32> {
    let mut category_id: Option<i32> = None;
    if category_exists_in_db(p, c) {
        let db = Connection::open(&real_path(p)).unwrap();

        let category_sql = format!("SELECT id FROM categories WHERE category = '{}' COLLATE NOCASE", c);
        category_id = if let Ok(value) = db.query_row(&category_sql, [], |row| row.get(0)) {
            Some(value)
        } else {
            None
        };
        
        category_id
    } else {
        category_id
    }
}

pub fn add_category_to_db(p: &str, c: &str) {
    if !category_exists_in_db(p, c) {
        let db = Connection::open(&real_path(p)).unwrap();

        let insert_query = format!("INSERT INTO categories(category) VALUES (?1)");

        if let Ok(mut statement) = db.prepare(&insert_query) {
            if let Err(error) = statement.execute([c]) {
                println!("{}", error);
            }
        }
    }
}

pub fn add_record_to_db(p: &str, r: &Record) {
    let category_id = if let Some(category) = &r.transaction.category {
        if !category.is_empty() && category.to_lowercase() != "uncategorized" {
            if let Some(id) = category_id(p, category) {
                Some(id)
            } else {
                add_category_to_db(p, category);
                category_id(p, &category)
            }
        } else {
            None
        }
    } else {
        None
    };

    let db = Connection::open(&real_path(p)).unwrap();

    let insert_statement = format!("INSERT INTO trades VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)");

    if let Ok(mut statement) = db.prepare(&insert_statement) {
        if let Err(error) = statement.execute(params![r.id.to_uppercase(),format!("{}", r.transaction.date.format("%Y-%m-%d")), r.transaction.check_number, r.transaction.vendor, r.transaction.memo, if let TransactionType::Deposit = r.transaction.transaction_type { r.transaction.amount.to_f64() } else { r.transaction.amount.to_f64()*-1.0 }, category_id, r.transaction.is_reconciled]) {
            println!("{}", error);
        }
    }
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
        if !category.is_empty() && category.to_lowercase() != "uncategorized" {
            if let Some(id) = category_id(p, category) {
                Some(id)
            } else {
                add_category_to_db(p, category);
                category_id(p, &category)
            }
        } else {
            None
        }
    } else {
        None
    };

    if let Some(record) = retrieve_record_with_id_from_db(p, &r.id) {
        if let Ok(db) = Connection::open(&real_path(p)) {
            if r.transaction != record.transaction {
                let update_statement = format!("UPDATE trades SET date = (?1), check_number = (?2), vendor = (?3), memo = (?4), amount = (?5), category = (?6), reconciled = (?7) WHERE id = (?8)");

                if let Ok(mut statement) = db.prepare(&update_statement) {
                    if let Err(error) = statement.execute(params![format!("{}", r.transaction.date.format("%Y-%m-%d")), r.transaction.check_number, r.transaction.vendor, r.transaction.memo, if let TransactionType::Deposit = r.transaction.transaction_type { r.transaction.amount.to_f64() } else { r.transaction.amount.to_f64()*-1.0 }, category_id, r.transaction.is_reconciled, record.id]) {
                        println!("{}", error);
                    }
                }
            }
        }
    }
}

pub fn delete_record_from_db(p: &str, i: &str) {
    if let Some(record) = retrieve_record_with_id_from_db(p, i) {
        if let Ok(db) = Connection::open(&real_path(p)) {
            let delete_statement = format!("DELETE FROM trades WHERE id = (?)");

            if let Ok(mut statement) = db.prepare(&delete_statement) {
                if let Err(error) = statement.execute(params![record.id]) {
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
    for (index, record) in r.iter().enumerate() {
        println!("Importing entry {} of {}", index+1, r.len());
        update_or_add_record(p, record);
    }
}