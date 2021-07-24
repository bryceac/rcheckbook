mod rcheckbook;
mod records;
mod subcommand;
mod list;

use rcheckbook::RCheckbook;
use clap::Clap;
use subcommand::SubCommand;

fn main() {
    let checkbook: RCheckbook = RCheckbook::parse();

    match checkbook.subcommand {
        SubCommand::List(l) => l.run()
    }
}