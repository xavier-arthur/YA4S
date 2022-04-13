mod url_handler;

use std::path::Path;
use rust::create_directories;
use rust::write_bytes;
use rust::Args;
use scraper::{Html, Selector};
use clap::Parser;
use url_handler::UrlHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // let url = format!("{}{}", BASE_URL, &args.url);
    let url = String::new();
    let response = reqwest::get(url).await?;

    let html = response.text().await?;

    let selector = Selector::parse("a.fileThumb").unwrap();
    let mut srcs = Vec::<String>::new();

    let thread_num = args.url
        .split("/")
        .collect::<Vec<&str>>()
        .pop()
        .unwrap();

    let target_path = format!("{}/{}", &args.destination, thread_num);
    
    create_directories(
        &thread_num,
        Path::new(&args.destination)
    );

    for img in Html::select(&Html::parse_document(&html), &selector) {
        if let Some(val) = img.value().attr("href") {
            srcs.push(
                format!("{}{}", "https://", &val[2..])
            );
        } 
    }

    for link in srcs {
        if args.verbose {
            println!("downloading {}", &link);
        }

        let img = reqwest::get(&link)
            .await?
            .bytes()
            .await?;

        let name_extract = link.split("/")
            .collect::<Vec<&str>>()
            .pop();

        let file_name = match name_extract {
            Some(val) => val,
            None => panic!("couldn't get file name of {}", link)
        };
        
        let write_result = write_bytes(
            &format!("{}/{}", &target_path, file_name), 
            &img
        );

        if let Err(err) = write_result {
            println!("Error while downloading {}\n{}", file_name, err);
        }
    }
    Ok(())
}