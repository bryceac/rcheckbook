use clap::Clap;
use crate::list::List;
use crate::add::Add;

use crate::remove::Remove;

#[derive(Clap)]
pub enum SubCommand {
    #[clap(version = "0.1", author = "Bryce Campbell")]
    Add(Add),
    #[clap(version = "0.1", author = "Bryce Campbell")]
    List(List),
    #[clap(version = "0.1", author = "Bryce Campbell")]
    Remove(Remove)
}