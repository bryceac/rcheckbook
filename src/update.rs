use bcheck::{ Record, Transaction, TransactionType, Save };
use clap::Clap;

#[derive(Clap)]
pub struct Update {
    pub file_path: String,
    
    #[clap(long, short)]
    pub id: String,

    #[clap(long, short)]
    pub date: String,

    #[clap(long)]
    pub check_number: Option<u32>,

    #[clap(long)]
    pub vendor: String,

    #[clap(long, short, default_value = "")]
    pub memo: String,

    #[clap(long, short, default_value = "0.0")]
    pub amount: f64,

    #[clap(long, short, default_value = "withdrawal")]
    pub transaction_type: TransactionType,

    #[clap(long, short)]
    pub reconciled: bool
}