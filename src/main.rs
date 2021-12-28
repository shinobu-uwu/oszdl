use crate::cli::Cli;
use config::Config;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use response::{Beatmap, Response};
use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, Write};
use std::path::PathBuf;
use structopt::StructOpt;

const APP_NAME: &str = "oszdl";
const CONFIG_NAME: &str = "config";
const BASE_URL: &str = "https://osu.ppy.sh/beatmapsets";
const SEARCH_URL: &str = "https://osu.ppy.sh/beatmapsets/search";

mod cli;
mod config;
mod response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config = load_config()?;
    let client = Client::new();
    let query = parse_args(&mut config);
    let response = send_request(&client, query.as_str(), &config).await?;

    display_maps(&response.beatmapsets);
    let maps_to_download = get_maps_to_download(response.beatmapsets)?;
    download_maps(&client, maps_to_download, &config).await?;

    Ok(())
}

fn load_config() -> Result<Config, Box<dyn Error>> {
    let mut config: Config = confy::load(APP_NAME, CONFIG_NAME)?;

    if config.download_directory.is_empty() {
        let mut download_directory = String::new();
        println!("Enter the path where you wish to save the downloaded beatmaps");
        stdin().read_line(&mut download_directory)?;
        config.download_directory = download_directory.trim().to_string();
    }

    if config.cookie.is_empty() {
        let mut cookie = String::new();
        println!("Enter your osu! session cookie (check README for more informations)");
        stdin().read_line(&mut cookie)?;
        config.cookie = cookie.trim().to_string();
    }

    confy::store(APP_NAME, CONFIG_NAME, &config)?;

    Ok(config)
}

fn parse_args(config: &mut Config) -> String {
    let cli = Cli::from_args();
    match cli.mode {
        Some(mode) => {
            let mode = mode as u8;
            config.filters.insert("m".to_string(), mode.to_string());
        }
        None => {}
    }

    if cli.recommended_difficulty {
        config
            .filters
            .insert("c".to_string(), "recommended".to_string());
    }

    cli.query
}

async fn send_request(
    client: &Client,
    query: &str,
    config: &Config,
) -> Result<Response, reqwest::Error> {
    let response = client
        .get(SEARCH_URL)
        .query(&[("q", query)])
        .query(&config.filters)
        .header("Cookie", &config.cookie)
        .send()
        .await?
        .json::<Response>()
        .await?;

    Ok(response)
}

fn display_maps(beatmapsets: &Vec<Beatmap>) {
    for (i, beatmapset) in beatmapsets.iter().enumerate() {
        println!("{}. {}", i + 1, beatmapset);
    }
}

fn get_maps_to_download(beatmapsets: Vec<Beatmap>) -> Result<Vec<Beatmap>, std::io::Error> {
    println!("Select the maps you wish to download (e.g 1 3-5 11)");
    let mut selection = String::new();
    stdin().read_line(&mut selection)?;
    let selection = selection.trim();

    let mut maps: Vec<Beatmap> = vec![];
    for s in selection.split(" ") {
        if s.contains('-') {
            let indices = s
                .split('-')
                .map(|n| n.parse::<usize>().unwrap() - 1)
                .collect::<Vec<usize>>();
            let selected = &beatmapsets[indices[0]..indices[1] + 1];
            maps.extend_from_slice(&selected);
            continue;
        }

        let index = s.parse::<usize>().unwrap() - 1;
        maps.push(beatmapsets[index].to_owned());
    }

    Ok(maps)
}

async fn download_maps(
    client: &Client,
    beatmapsets: Vec<Beatmap>,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    for map in beatmapsets.iter() {
        let url = format!("{}/{}/download", BASE_URL, map.id);
        let response = client
            .get(url)
            .header("Cookie", &config.cookie)
            .header("Referer", format!("{}/{}", BASE_URL, map.id))
            .send()
            .await?;
        let total_size = response.content_length().unwrap();

        let progress = ProgressBar::new(total_size);
        progress.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .progress_chars("#>-"));
        progress.set_message(format!("Downloading {}", map));

        let path = PathBuf::from(&config.download_directory)
            .join(PathBuf::from(format!(
            "{}-{}.osz",
            map.id,
            map.sanitized_name()
        )));

        let mut file = File::create(path)?;
        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item.or(Err(format!("Error while downloading file")))?;
            file.write(&chunk)
                .or(Err(format!("Error while writing to file")))?;
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            progress.set_position(new);
        }

        progress.finish_with_message("Finished!");
    }

    Ok(())
}
