use clap::Parser;
use crate::list::List;
use crate::add::Add;
use crate::create::Create;
use crate::remove::Remove;
use crate::update::Update;

#[derive(Clap)]
pub enum SubCommand {
    #[clap(version = "0.1", author = "Bryce Campbell")]
    Add(Add),
    #[clap(version = "0.1", author = "Bryce Campbell")]
    Create(Create),
    #[clap(version = "0.1", author = "Bryce Campbell")]
    List(List),
    #[clap(version = "0.1", author = "Bryce Campbell")]
    Remove(Remove),
    #[clap(version = "0.1.1", author = "Bryce Campbell")]
    Update(Update)
}