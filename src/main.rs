mod rcheckbook;
mod records;
mod subcommand;
mod add;
mod list;

use rcheckbook::RCheckbook;
use clap::Clap;
use subcommand::SubCommand;

fn main() {
    let checkbook: RCheckbook = RCheckbook::parse();

    match checkbook.subcommand {
        SubCommand::Add(a) => a.run().unwrap(),
        SubCommand::List(l) => l.run()
    }
}