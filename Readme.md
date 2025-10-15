# Random anime wallpaper

### About

This is a small tool that sets a random anime wallpaper every hour.

It picks a random anime from your config file at `~/.config/anime_list.toml` and fetches a random wallpaper from `wallhaven.cc`.

Currently, this only works on gnome, but it can probably be adapted to set the wallpaper for whatever window manager/DE you're using without much effort.

The random wallpaper is saved as `wallpaper.jpg`.

### Build and installation

To build a production-optimized binary and install it system-wide:

Compile the code in release mode:

```bash
cargo build --release
```

Install the binary to your system (this may require `sudo`):

```bash
sudo cp target/release/anime_wallpaper /usr
/local/bin
```


You can copy the provided `anime_list.example.toml` to `~/.config/anime_list.toml` as a starting point:

```bash
cp anime_list.example.toml ~/.config/anime_list.toml
```

#### Config options

The config file supports the following options:

- `animes`: List of anime titles to search for wallpapers.
- `purity`: (optional) Wallhaven purity code, default: `100`.
- `resolution`: (optional) Desired wallpaper resolution, default: `3840x2160`.
- `filename`: (optional) Output filename, default: `wallpaper.jpg`.
- `directory`: (optional) Output directory, default: `${CACHE_DIR}`.
- `auth`: (optional) Wallhaven API key, default: none.
- `desktop`: (optional) Desktop environment, default: `Gnome`.
- `page`: (optional) Page number to fetch from Wallhaven (default: random 1-10).
- `sorting`: (optional) Sorting method for results, e.g. `views`, `random`, `favorites`, `toplist` (default: `views`).
- `order`: (optional) Order for sorting, either `desc` or `asc` (default: `desc`).

See the example config for more details.

### Systemd Service and Timer

Copy the systemd service and timer files:

```bash
sudo cp anime_wallpaper.service /etc/systemd/system/
sudo cp anime_wallpaper.timer /etc/systemd/system/
```

Reload systemd to recognize the new files:

```bash
sudo systemctl daemon-reload
```

Enable and start the timer:

```bash
sudo systemctl enable anime_wallpaper.timer
sudo systemctl start anime_wallpaper.timer
```

---
