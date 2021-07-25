use bcheck::{ Record, Transaction, TransactionType, Save };
use crate::records::Records;
use clap::Clap;

#[derive(Clap)]
pub struct Update {
    pub file_path: String,
    
    #[clap(long, short)]
    pub id: String,

    #[clap(long, short)]
    pub date: Option<String>,

    #[clap(long)]
    pub check_number: Option<u32>,

    #[clap(long)]
    pub vendor: Option<String>,

    #[clap(long, short)]
    pub memo: Option<String>,

    #[clap(long, short)]
    pub amount: Option<f64>,

    #[clap(long, short)]
    pub transaction_type: Option<String>,

    #[clap(long, short)]
    pub reconciled: bool
}

impl Update {
    pub fn run(&self) -> Result<(), String> {
        if self.file_path.starts_with("~") {
            let modified_path = shellexpand::tilde(&self.file_path).into_owned();
    
            self.add_record_to(&modified_path)
        } else {
            match fs::canonicalize(self.file_path.clone()) {
                Ok(real_path) => if let Some(file_path) = real_path.to_str() {
                    self.add_record_to(file_path)
                } else {
                    Err(String::from("File path could not be recognized"))
                },
                Err(error) => Err(error.to_string())
            }
        }
    }

    fn update_record(&self, p: &str) -> Result<(), String> {
        let mut stored_records = Records::from_file(p)?;

        if let Some(mut record) = stored_records.record_matching_id(self.id) {
            if let Some(date_string) = self.date {
                // implement later
            }

            if let Some(check_number) = self.check_number {

            }
            
            if let Some(vendor) = self.vendor {

            }

            if let Some(memo) = self.memo {

            }

            if let Some(amount) = self.amount {

            }

            if let Some(transaction_type) = self.transaction_type {

            }

            if self.reconciled {

            }

        }
    }
}