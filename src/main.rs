mod rcheckbook;
mod records;
mod subcommand;
mod add;
mod create;
mod list;
mod remove;
mod update;
mod database;

use rcheckbook::RCheckbook;
use clap::Parser;
use subcommand::SubCommand;

fn main() {
    let checkbook: RCheckbook = RCheckbook::parse();

    match checkbook.subcommand {
        SubCommand::Add(a) => a.run(),
        SubCommand::Create(c) => c.run().unwrap(),
        SubCommand::List(l) => l.run(),
        SubCommand::Remove(r) => r.run().unwrap(),
        SubCommand::Update(u) => u.run().unwrap()
    }
}