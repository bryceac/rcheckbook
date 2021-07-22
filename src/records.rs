use bcheck::{ Record, TransactionType };

pub struct Records {
    records: Vec<Record>
}

impl Records {
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

    fn record_before(&self, record: &Record) -> Option<Record> {
        if let Some(index) = self.sorted_records().iter().position(|r| r == record) {
            if index > 0 { Some(self.sorted_records()[index-1].clone()) } else { None }
        } else {
            None
        }
    }

    fn retrieve_previous_records_for_records(&mut self) {
        let mut stored_records = self.records.clone();

        for record in stored_records.iter_mut() {
            if let Some(previous_record) = self.record_before(&record) {
                record.previous_record = Some(Box::new(previous_record));
            }
        }

        self.records = stored_records;
    }

    pub fn display(&mut self) {
        self.retrieve_previous_records_for_records();

        for record in self.sorted_records() {
            println!("{}", string_for_record(&record))
        }
    }
}

fn string_for_record(record: &Record) -> String {
    match record.transaction.transaction_type {
        TransactionType::DEPOSIT => if let Some(check_number) = record.transaction.check_number {
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
        },
        TransactionType::WITHDRAWAL => if let Some(check_number) = record.transaction.check_number {
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