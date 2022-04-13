use std::{fs::File, io::Write};
use clap::Parser;
use std::path::Path;
use regex::Regex;

use std::fs::{
    create_dir as mkdir,
};

pub fn write_bytes(path: &str, blob: &[u8]) -> Result<(), std::io::Error>{
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(err) => panic!("\npanicked while creating file: {}\nerror: {}", path, err)
    };

    file.write_all(&blob)
}

pub fn create_directories(name: &str, path: &Path) -> () {
    if !Path::exists(&path.join(name)) {
        match mkdir(path.join(name)) {
            Err(err) => panic!("\npanicked while creating directories: {}\npath: {}", err, path.display()),
            Ok(()) => {}
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    pub url: String,

    #[clap(short, long, default_value = ".")]
    pub destination: String,

    #[clap(long)]
    pub verbose: bool,
}