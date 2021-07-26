use clap::Clap;
use crate::subcommand::SubCommand;

#[derive(Clap)]
#[clap(version = "0.2.1", author = "Bryce Campbell <tonyhawk2100@gmail.com>")]
pub struct RCheckbook {

    #[clap(subcommand)]
    pub subcommand: SubCommand
}