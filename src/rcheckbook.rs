use clap::Parser;
use crate::subcommand::SubCommand;

#[derive(Parser)]
#[clap(version = "0.4.2", author = "Bryce Campbell <tonyhawk2100@gmail.com>")]
pub struct RCheckbook {

    #[clap(subcommand)]
    pub subcommand: SubCommand
}