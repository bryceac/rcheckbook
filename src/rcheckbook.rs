use clap::Parser;
use crate::subcommand::SubCommand;

#[derive(Parser)]
#[clap(version = "0.4.4", author = "Bryce Campbell <tonyhawk2100@gmail.com>")]
pub struct RCheckbook {

    #[clap(subcommand)]
    pub subcommand: SubCommand
}