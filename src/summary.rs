use clap::Parser;
use crate::records::Records;
use crate::database::*;

#[derive(Parse)]
pub struct Summary {
    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,

    #[clap(arg_enum, default_value=Period::All)]
    pub period: Period
}