use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "Bryce Campbell <tonyhawk2100@gmail.com>")]
pub struct RCheckbook {
    pub file_path: String
}