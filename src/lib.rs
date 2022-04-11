use std::{fs::File, io::Write};
use clap::Parser;

pub fn write_bytes(path: &str, blob: &[u8]) {
    let mut file = File::create(path).unwrap();
    file.write_all(&blob);
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    pub url: String,

    #[clap(short, long)]
    pub destination: Option<String>,

    #[clap(long)]
    pub verbose: bool,
}