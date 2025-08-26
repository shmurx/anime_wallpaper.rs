# Random anime wallpaper

### About

This is a small tool that sets a random anime wallpaper every hour.

It picks a random anime in `anime_list.toml` and fetches a random wallpaper from `wallhaven.cc`.

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
sudo cp /target/release/anime_wallpaper /usr/local/bin/
```

Copy the example anime list to `~/.config/` and adapt to your preferences:

```bash
cp anime_list.toml ~/.config
```

Now you can run `anime_wallpaper` from anywhere on your system.

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
