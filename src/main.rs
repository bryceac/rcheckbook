mod rcheckbook;
mod records;
mod subcommand;
mod add;
mod import;
mod export;
mod list;
mod remove;
mod update;
mod database;
mod shared;

use rcheckbook::RCheckbook;
use clap::Parser;
use subcommand::SubCommand;

fn main() {
    let checkbook: RCheckbook = RCheckbook::parse();

    match checkbook.subcommand {
        SubCommand::Add(a) => a.run(),
        SubCommand::Import(i) => i.run(),
        SubCommand::Export(e) => e.run(),
        SubCommand::List(l) => l.run(),
        SubCommand::Remove(r) => r.run(),
        SubCommand::Update(u) => u.run()
    }
}