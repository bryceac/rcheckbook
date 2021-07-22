mod rcheckbook;
mod records;

use rcheckbook::RCheckbook;
use clap::Clap;

fn main() {
    let checkbook: RCheckbook = RCheckbook::parse();

    if checkbook.file_path.starts_with("~") {

    } else {

    }
}