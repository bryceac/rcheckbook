use bcheck::{ Record, TransactionType };

pub struct Records {
    records: Vec<Record>
}

impl Records {
    pub fn new() -> Records {
        Records {
            records: Vec::new()
        }
    }
    
    pub fn from_file(path: &str) -> Result<Records, String> {
        match Record::from_file(path) {
            Ok(records) => Ok(Records { records }),
            Err(error) => Err(error)
        }
    }
    
    pub fn sorted_records(&self) -> Vec<Record> {
        let mut stored_records = self.records.clone();
        stored_records.sort_by_key(|r| r.transaction.date);
        
        stored_records
    }

    pub fn add(&mut self, record: Record) {
        self.records.push(record)
    }

    pub fn remove(&mut self, record_id: String) {
        if let Some(index) = self.records.iter().position(|r| r.id == record_id) {
            self.records.remove(index);
        }
    }

    pub fn update(&mut self, record_id: String, record: Record) {
        if let Some(index) = self.records.iter().position(|r| r.id == record_id) {
            self.records[index] = record
        }
    }

    fn record_before(&self, record: &Record) -> Option<Record> {
        if let Some(index) = self.sorted_records().iter().position(|r| r == record) {
            if index > 0 { Some(self.sorted_records()[index-1].clone()) } else { None }
        } else {
            None
        }
    }

    pub fn record_matching_id(&self, id: String) -> Option<Record> {
        if let Some(record) = self.records.iter().find(|r| r.id == id) {
            Some(record.clone())
        } else {
            None
        }
    }

    fn retrieve_previous_records_for_records(&mut self) {
        let mut stored_records = self.records.clone();

        for record in stored_records.iter_mut() {
            if let Some(previous_record) = self.record_before(&record) {
                record.previous_record = Some(Box::new(previous_record));

                if let Some(index) = self.records.iter().position(|r| r == record) {
                    self.records[index] = record.clone();
                }
            }
        }
    }

    pub fn display(&mut self, with_id: bool) {
        self.retrieve_previous_records_for_records();

        for record in self.sorted_records() {
            println!("{}", string_for_record(&record, with_id))
        }
    }
}

fn string_for_record(record: &Record, with_id: bool) -> String {
    match record.transaction.transaction_type {
        TransactionType::Deposit => if let Some(check_number) = record.transaction.check_number {
            if with_id {
                format!("{}\t{}\t{}\t{}\t{}; {}\t{:.2}\tx\t{:.2}",
                record.id,
                record.transaction.date.format("%m/%d/%Y"),
                check_number,
                if record.transaction.is_reconciled { "Y" } else { "N" },
                if record.transaction.vendor.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.vendor.clone()
                },
                if record.transaction.memo.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.memo.clone()
                },
                record.transaction.amount,
                record.balance())
            } else {
                format!("{}\t{}\t{}\t{}; {}\t{:.2}\tx\t{:.2}",
                record.transaction.date.format("%m/%d/%Y"),
                check_number,
                if record.transaction.is_reconciled { "Y" } else { "N" },
                if record.transaction.vendor.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.vendor.clone()
                },
                if record.transaction.memo.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.memo.clone()
                },
                record.transaction.amount,
                record.balance())
            }
        } else {
            if with_id {
                format!("{}\t{}\tx\t{}\t{}; {}\t{:.2}\tx\t{:.2}",
                record.id,
                record.transaction.date.format("%m/%d/%Y"),
                if record.transaction.is_reconciled { "Y" } else { "N" },
                if record.transaction.vendor.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.vendor.clone()
                },
                if record.transaction.memo.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.memo.clone()
                },
                record.transaction.amount,
                record.balance())
            } else {
                format!("{}\tx\t{}\t{}; {}\t{:.2}\tx\t{:.2}",
                record.transaction.date.format("%m/%d/%Y"),
                if record.transaction.is_reconciled { "Y" } else { "N" },
                if record.transaction.vendor.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.vendor.clone()
                },
                if record.transaction.memo.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.memo.clone()
                },
                record.transaction.amount,
                record.balance())
            }
        },
        TransactionType::Withdrawal => if let Some(check_number) = record.transaction.check_number {
            if with_id {
                format!("{}\t{}\t{}\t{}\t{}; {}\tx\t{:.2}\t{:.2}",
                record.id,
                record.transaction.date.format("%m/%d/%Y"),
                check_number,
                if record.transaction.is_reconciled { "Y" } else { "N" },
                if record.transaction.vendor.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.vendor.clone()
                },
                if record.transaction.memo.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.memo.clone()
                },
                record.transaction.amount,
                record.balance())
            } else {
                format!("{}\t{}\t{}\t{}; {}\tx\t{:.2}\t{:.2}",
                record.transaction.date.format("%m/%d/%Y"),
                check_number,
                if record.transaction.is_reconciled { "Y" } else { "N" },
                if record.transaction.vendor.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.vendor.clone()
                },
                if record.transaction.memo.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.memo.clone()
                },
                record.transaction.amount,
                record.balance())
            }
        } else {
            if with_id {
                format!("{}\t{}\tx\t{}\t{}; {}\tx\t{:.2}\t{:.2}",
                record.id,
                record.transaction.date.format("%m/%d/%Y"),
                if record.transaction.is_reconciled { "Y" } else { "N" },
                if record.transaction.vendor.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.vendor.clone()
                },
                if record.transaction.memo.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.memo.clone()
                },
                record.transaction.amount,
                record.balance())
            } else {
                format!("{}\tx\t{}\t{}; {}\tx\t{:.2}\t{:.2}",
                record.transaction.date.format("%m/%d/%Y"),
                if record.transaction.is_reconciled { "Y" } else { "N" },
                if record.transaction.vendor.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.vendor.clone()
                },
                if record.transaction.memo.is_empty() {
                    String::from("x")
                } else {
                    record.transaction.memo.clone()
                },
                record.transaction.amount,
                record.balance())
            }
        }
    }
}