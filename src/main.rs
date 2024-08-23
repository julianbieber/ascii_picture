use std::{fs::OpenOptions, io, thread, time::Duration};

use arboard::Clipboard;
use clap::Parser;
use image::{imageops::FilterType, DynamicImage, GenericImageView};

#[derive(Parser)]
struct Opt {
    #[clap(long)]
    url: String,
    #[clap(long)]
    width: u32,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    let image_bytes = if opt.url.starts_with("http") {
        download(opt.url.as_str()).await
    } else {
        std::fs::read(opt.url).unwrap()
    };

    let i = image::load_from_memory(&image_bytes).unwrap();
    let ascii = to_ascii(i, opt.width);
    let mut clipboard = Clipboard::new().unwrap();
    println!("{ascii}");
    clipboard.set_text(ascii).unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}

async fn download(url: &str) -> Vec<u8> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await.unwrap();
    response
        .error_for_status()
        .unwrap()
        .bytes()
        .await
        .unwrap()
        .to_vec()
}

fn to_ascii(image: DynamicImage, width: u32) -> String {
    let (original_width, original_height) = dbg!(image.dimensions());
    let scaling = original_width / width;
    let height = original_height / scaling;
    dbg!((width, height));

    let image = image.resize(width, height, FilterType::Nearest);
    let image = image.grayscale();
    let (actual_width, _) = image.dimensions();
    image
        .pixels()
        .map(|(x, y, color)| {
            let c = color.0[0];
            let symbol = if c < 64 {
                "  "
            } else if c < 128 {
                "||"
            } else if c < 128 + 64 {
                "##"
            } else {
                "HH"
            };
            if x == actual_width - 1 {
                format!("{symbol}\n")
            } else {
                symbol.to_string()
            }
        })
        .collect()
}
