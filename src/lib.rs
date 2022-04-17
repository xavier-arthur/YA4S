use std::{fs::File, io::Write};
use clap::Parser;
use std::path::Path;
use std::fs::{
    create_dir as mkdir,
};

pub fn write_bytes(path: &Path, blob: &[u8]) -> Result<(), std::io::Error> {
    match File::create(path) {
        Ok(mut file) => file.write_all(&blob),
        Err(err) => panic!("\npanicked while creating file: {}\nerror: {}", path.display(), err)
    }
}

pub fn create_directories(name: &str, path: &Path) -> () {
    if !Path::exists(&path.join(name)) {
        match mkdir(path.join(name)) {
            Err(err) => panic!("\npanicked while creating directories: {}\npath: {}", err, path.display()),
            _ => {}
        }
    }
}

#[derive(Parser, Debug)]
#[clap(name = "yaas", author = "Arthur X", version = "1.0", about = "A image board scrapper", long_about = None)]
pub struct Args {
    pub url: String,

    #[clap(short, long, default_value = ".")]
    pub output: String,

    #[clap(long)]
    pub verbose: bool,
}