use rust::write_bytes;
use rust::Args;
use scraper::{Html, Selector, selector};
use clap::Parser;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }

/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("http://boards.4channel.org/g")
        .await?;

    let html = response.text().await?;
    let selector = Selector::parse("img[src]").unwrap();
    let mut sources = Vec::<String>::new();

    for img in Html::select(&Html::parse_document(&html), &selector) {
        if let Some(val) = img.value().attr("src") {
            sources.push(val[2..].to_string());
        }
    }

    let mut file_name: &str;
    let mut path: String;

    for mut link in sources {
        path = String::from("/tmp/rust/");

        link = format!("{}{}", "https://", link);

        let mut splitted_link = link.split("/")
            .collect::<Vec<&str>>();

        let image_response = reqwest::get(&link)
            .await?
            .bytes()
            .await?;

        file_name = splitted_link.pop().unwrap();
        path.push_str(file_name);

        write_file(&path, &image_response);
    }

    Ok(())
}
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const BASE_URL: &str = "https://boards.4channel.org";
    const TARGET_PATH: &str = "/tmp/rust/";

    let args = Args::parse();
    let url  = format!("{}{}", BASE_URL, &args.url);
    let response = reqwest::get(url).await?;

    let html = response.text().await?;

    let selector = Selector::parse("img[src][data-md5]").unwrap();
    let mut srcs = Vec::<String>::new();

    for img in Html::select(&Html::parse_document(&html), &selector) {
        if let Some(val) = img.value().attr("src") {
            srcs.push(
                format!("{}{}", "https://", &val[2..])
            );
        } 
    }

    for link in srcs {
        let img = reqwest::get(&link)
            .await?
            .bytes()
            .await?;

        // file_name = link.split("/").collect::<Vec<&str>>().pop().unwrap();

        let name_extract = link.split("/")
            .collect::<Vec<&str>>()
            .pop();

        let file_name = match name_extract {
            Some(val) => val,
            None => panic!("couldn't get file name of {}", link)
        };

        if args.verbose {
            println!("writing {}", &file_name);
        }

        // println!("{}{}", TARGET_PATH, file_name);
        
        write_bytes(
            &format!("{}{}", TARGET_PATH, file_name), 
            &img
        );
    }

    Ok(())
}