use clap::Parser;
use crate::subcommand::SubCommand;

#[derive(Clap)]
#[clap(version = "0.2.2", author = "Bryce Campbell <tonyhawk2100@gmail.com>")]
pub struct RCheckbook {

    #[clap(subcommand)]
    pub subcommand: SubCommand
}