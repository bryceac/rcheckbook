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
    Import(Import),
    Export(Export),
    List(List),
    Remove(Remove),
    Update(Update),
    Summary(Summary)
}