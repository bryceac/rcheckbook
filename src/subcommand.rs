use clap::Parser;
use crate::list::List;
use crate::add::Add;
use crate::remove::Remove;
use crate::update::Update;
use crate::import::Import;
use crate::export::Export;
use crate::summary::Summary;

#[derive(Parser)]
pub enum SubCommand {
    Add(Add),
    #[clap(version = "0.4", author = "Bryce Campbell")]
    Import(Import),
    #[clap(version = "0.4", author = "Bryce Campbell")]
    Export(Export),
    #[clap(version = "0.4", author = "Bryce Campbell")]
    List(List),
    #[clap(version = "0.3", author = "Bryce Campbell")]
    Remove(Remove),
    #[clap(version = "0.3", author = "Bryce Campbell")]
    Update(Update),
    #[clap(version = "0.1", author = "Bryce Campbell")]
    Summary(Summary)
}