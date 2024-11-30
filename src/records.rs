use bcheck::Record;
use crate::database::*;

pub struct Records {
    records: Vec<Record>
}

impl Records {
    pub fn from(r: Vec<Record>) -> Records {
        Records { 
            records: r 
        }
    }
    
    pub fn sorted_records(&self) -> Vec<Record> {
        let mut stored_records = self.records.clone();
        stored_records.sort_by_key(|r| r.transaction.date);
        
        stored_records
    }

    pub fn display(&mut self, db: &str) {
        for record in self.sorted_records() {
            let balance = self.balance_for_record(db, &record);
            println!("{}\t{:.2}", record, balance);
        }
    }

    pub fn filtered_display(&mut self, category: Option<String>, db: &str) {
        if let Some(category) = category {
            let filtered_records: Vec<Record> = self.sorted_records().into_iter().filter(|record| record.transaction.category.clone().unwrap_or("Uncategorized".to_string()).to_lowercase() == category.to_lowercase()).collect();

            for record in filtered_records {
                let balance = self.balance_for_record(db, &record);
                println!("{}\t{:.2}", record, balance);
            }
        } else {
            println!("{}", "Nothing matched the specified criteria")
        }
    }

    fn balance_for_record(&self, db: &str, r: &Record) -> f64 {
        retrieve_balance_for_record(db, r.clone())
    }
}