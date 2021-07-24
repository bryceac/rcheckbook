use clap::Clap;
use crate::list::List;

#[derive(Clap)]
pub enum SubCommand {
    #[clap(version = "0.1", author = "Bryce Campbell")]
    List(List)
}