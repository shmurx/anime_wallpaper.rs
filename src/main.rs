use std::fs::File;
use std::io::copy;
use std::path::{Path, PathBuf};
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
enum Desktop {
    Gnome,
    Custom {
        command: String,
        args: Option<Vec<String>>
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct AnimeConfig {
    animes: Vec<String>,
    purity: String,
    resolution: String,
    filename: String,
    directory: Option<String>,
    auth: Option<String>,
    desktop: Desktop
}

impl Default for AnimeConfig {
  fn default() -> Self {
     Self {
         animes: vec![],
         purity: "100".to_string(),
         resolution: "3840x2160".to_string(),
         filename: "wallpaper.jpg".to_string(),
         directory: None,
         auth: None,
         desktop: Desktop::Gnome
     }
  }
}

fn load_config(path: &str) -> Result<AnimeConfig, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config: AnimeConfig = toml::from_str(&contents)?;
    Ok(config)
}

fn prepare_output_file(cfg: &AnimeConfig) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dir  = match &cfg.directory {
        &Some(ref out) => PathBuf::from(out),
        &None => {
            dirs::home_dir()
                .ok_or("Could not determine home directory")?
                .join(".cache/anime_wallpaper")
        }
    };
    fs::create_dir_all(&dir)?;
    Ok(dir.join(&cfg.filename))
}

fn download_wallpaper_wallhaven(
    query: &str,
    config: &AnimeConfig,
    save_path: &Path
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!(
        "https://wallhaven.cc/api/v1/search?q={}&categories=010&purity={}&resolutions={}&sorting=views&order=des{}",
        urlencoding::encode(query),
        config.purity,
        config.resolution,
        match &config.auth {
            &Some(ref auth) => format!("&apikey={}", auth),
            &None => String::new()
        }
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
    println!("Saved wallpaper to {}", save_path.display());
    Ok(())
}

fn set_wallpaper(save_path: PathBuf, config: AnimeConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting wallpaper using {:?}", config.desktop);
    let absolute_path = save_path.canonicalize()?;
    let path_str = absolute_path.to_str().ok_or("Invalid path")?;
    match config.desktop {
        Desktop::Gnome => {
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
                    "wallpaper",
                ])
                .status()?;
        },
        Desktop::Custom { command, args: None } => {
            Command::new(command).arg(path_str).status()?;
        }
        Desktop::Custom { command, args: Some(args) } => {
           Command::new(command).arg(path_str)
                .args(args)
                .status()?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = dirs::home_dir()
        .ok_or("Could not determine home directory")?
        .join(".config/anime_list.toml");
    let config = load_config(config_path.to_str().ok_or("Invalid config path")?)?;

    if config.animes.is_empty() {
        eprintln!("Anime list is empty.");
        return Ok(());
    }

    let random_title = config.animes.choose(&mut thread_rng()).unwrap();
    println!("Searching wallpaper for '{}'", random_title);

    let output = prepare_output_file(&config)?;
    download_wallpaper_wallhaven(random_title, &config, &output)?;
    set_wallpaper(output, config)?;
    println!("Wallpaper set from Wallhaven!");
    Ok(())
}
