use std::path::Path;
use rust::create_directories;
use rust::write_bytes;
use rust::Args;
use scraper::{Html, Selector};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let response = reqwest::get(&args.url).await?;
    let html = response.text().await?;

    let selector = Selector::parse("a.fileThumb").unwrap();
    let mut srcs = Vec::<String>::new();

    let thread_num = args.url
        .split("/")
        .collect::<Vec<&str>>()
        .pop()
        .unwrap();

    let formatted_path = &format!("{}/{}", &args.output, thread_num);
    let target_path = Path::new(formatted_path);

    create_directories(
        &thread_num,
        Path::new(&args.output)
    );

    for img in Html::select(&Html::parse_document(&html), &selector) {
        if let Some(val) = img.value().attr("href") {
            srcs.push(
                format!("{}{}", "https://", &val[2..])
            );
        } 
    }

    let mut file_name: &str;

    for link in srcs {
        file_name = match link.split("/") .collect::<Vec<&str>>().pop() {
            Some(val) => val,
            None => panic!("couldn't get file name of {}", link)
        };

        let full_path = target_path.join(file_name);

        if full_path.exists() {
            println!("{} exists, skipping...", full_path.display());
            continue;
        }

        if args.verbose {
            println!("downloading {}", &link);
        }

        let img = reqwest::get(&link).await?
            .bytes()
            .await?;

        let write_result = write_bytes(
            &full_path,
            &img
        );

        if let Err(err) = write_result {
            println!("Error while downloading {}\n{}", file_name, err);
        }
    }
    Ok(())
}