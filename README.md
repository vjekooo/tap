# Terminal Audio Player (TAP)

A small terminal audio player written in Rust, with a built-in file browser and tag support.

## Features

- Play MP3, WAV, FLAC and OGG files
- Browse and open tracks from inside the UI
- Shows artist and title from file tags

## Install

Download a prebuilt binary from the [Releases](https://github.com/vjekooo/tap/releases) page.

Or build from GitHub (requires [Rust](https://rustup.rs)):

```bash
cargo install --git https://github.com/vjekooo/tap
```

On Linux, building needs the ALSA development files:

```bash
sudo apt install libasound2-dev
```

## Usage

```bash
tap                 # open the file browser
tap song.mp3        # play a file
tap ~/Music         # open the browser in a folder
```

## Controls

| Key | Action |
|---|---|
| `O` | Open file browser |
| `Space` | Play / pause |
| `S` | Stop |
| `←` / `→` | Seek 10 seconds back / forward |
| `↑` / `↓` | Volume up / down |
| `Q` | Quit |

In the browser: `↑`/`↓` (or `j`/`k`) to move, `Enter` to open, `Backspace` to go up, `Esc` to return to the player.

## License

MIT
