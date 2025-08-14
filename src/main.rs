use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::process::Command;

use rand::seq::SliceRandom;
use rand::thread_rng;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct WallhavenResponse {
    data: Vec<WallpaperData>,
}

#[derive(Deserialize)]
struct WallpaperData {
    path: String,
}

use std::fs;

#[derive(Debug, Deserialize)]
struct AnimeConfig {
    animes: Vec<String>,
}

fn load_config(path: &str) -> Result<AnimeConfig, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config: AnimeConfig = toml::from_str(&contents)?;
    Ok(config)
}

fn download_wallpaper_wallhaven(
    query: &str,
    save_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!(
        "https://wallhaven.cc/api/v1/search?q={}&categories=010&purity=110&resolutions=3840x2160&sorting=views&order=des",
        query
    );

    let resp = client.get(&url).send()?.json::<WallhavenResponse>()?;

    let wallpaper = resp
        .data
        .choose(&mut thread_rng())
        .ok_or("No wallpaper found")?;
    let wallpaper_url = &wallpaper.path;

    let mut response = client.get(wallpaper_url).send()?;
    let mut file = File::create(save_path)?;
    copy(&mut response, &mut file)?;

    let absolute_path = save_path.canonicalize()?;
    let path_str = absolute_path.to_str().ok_or("Invalid path")?;

    for key in ["picture-uri", "picture-uri-dark"] {
        Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.background",
                key,
                &format!("file://{}", path_str),
            ])
            .status()?;
    }

    Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.background",
            "picture-options",
            "centered",
        ])
        .status()?;

    println!("Wallpaper set from Wallhaven!");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config("anime_list.toml")?;
    let anime_titles = config.animes;

    if anime_titles.is_empty() {
        eprintln!("Anime list is empty.");
        return Ok(());
    }

    let random_title = anime_titles.choose(&mut thread_rng()).unwrap();
    println!("Searching wallpaper for: {}", random_title);

    let save_path = Path::new("wallpaper.jpg");
    download_wallpaper_wallhaven(random_title, save_path)?;
    println!("Success!");
    Ok(())
}
