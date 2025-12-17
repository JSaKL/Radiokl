# Radiokl

> A simple command-line application for searching and playing internet radio stations

Radiokl is a CLI application for Linux and Mac that lets you search, play, and manage internet radio stations directly from your terminal with an interactive TUI interface.

## Features

- 🔍 **Search radio stations** by name, country, and language
- 🎵 **Stream audio** from thousands of stations worldwide
- ⭐ **Save favorites** for quick access to your preferred stations
- 🎛️ **Interactive TUI** with keyboard controls
- 🖥️ **Client-server architecture** for efficient resource management
- 🌐 **Powered by radio-browser API** for extensive station database

## Description

The software consists of two components:
- **radio_client**: The front-end CLI/TUI interface for user interaction
- **radio_server**: The back-end server handling API requests and station data

The server component uses the [radio-browser API](https://api.radio-browser.info/) to discover available servers via DNS lookup (`all.api.radio-browser.info`) and searches for radio stations across the internet. Audio streaming is handled by **FFplay** media player (part of FFmpeg).

## Getting Started

### Prerequisites

This software works on **Linux and macOS** only.

**Required:**
- [FFmpeg](https://ffmpeg.org/) (includes FFplay for audio streaming)
- [Rust](https://www.rust-lang.org/) (for building from source)

To verify FFmpeg is installed:
```bash
ffplay -version
```

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/Radiokl.git
   cd Radiokl
   ```

2. **Install FFmpeg** (if not already installed)
   
   **macOS:**
   ```bash
   brew install ffmpeg
   ```
   
   **Linux (Ubuntu/Debian):**
   ```bash
   sudo apt-get install ffmpeg
   ```

3. **Build the project**
   ```bash
   cargo build --release
   ```

4. **Executables location**
   
   After building, executables will be located in:
   ```
   target/release/radio_client
   target/release/radio_server
   ```

### Usage

The client automatically starts the server if it's not already running. You can also start the server manually first if preferred.

#### Basic Commands

**View help:**
```bash
./target/release/radio_client -h
```

**Search for stations:**
```bash
# Search by name
./target/release/radio_client search jazz

# Search by country
./target/release/radio_client -c Finland search

# Search by language
./target/release/radio_client -l english search rock

# Combine filters
./target/release/radio_client -c France -l french search pop
```

**Access favorites:**
```bash
./target/release/radio_client favs
```

**Stop playback:**
```bash
./target/release/radio_client stop
```

**Stop the server:**
```bash
./target/release/radio_client stop-server
```

#### CLI Options

| Option | Short | Long | Description | Default |
|--------|-------|------|-------------|---------|
| Country | `-c` | `--country` | Filter by country | - |
| Language | `-l` | `--language` | Filter by language | - |
| Address | `-a` | `--addr` | Server connection address | `localhost:8080` |
| Help | `-h` | `--help` | Print help information | - |
| Version | `-V` | `--version` | Print version | - |

#### Interactive TUI Controls

Once in the station list view, use these keyboard shortcuts:

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate through stations |
| `Ctrl-p` | Play selected station |
| `Ctrl-s` | Stop playback |
| `Ctrl-w` | Save station to favorites |
| `Ctrl-d` | Delete from favorites (when in favs menu) |
| `Ctrl-q` | Quit |

#### Example Session

```bash
$ ./target/release/radio_client search jazz
```

**Output:**
```
(585 stations found) Playing now: -
1: 101 SMOOTH JAZZ, Country: The United States Of America, Language: english
2: Adroit Jazz Underground, Country: The United States Of America, Language: english
3: Adroit Jazz Underground HD Opus, Country: The United States Of America, Language: 
4: Jazz Radio Blues, Country: France, Language: french
5: Radio Swiss Jazz, Country: Switzerland, Language: 
6: Instrumental Jazz, Country: The Russian Federation, Language: russian
7: Bossa Jazz Brasil, Country: Brazil, Language: brazilian portuguese
8: SMOOTH JAZZ 24/7, Country: The United States Of America, Language: english
...
>>> Options: Ctrl-p to Play, Ctrl-s to Stop, Ctrl-w to Save, Ctrl-q to Quit
```

## Troubleshooting

### Terminal window too narrow
**Issue:** The TUI requires a wide terminal window to display properly.

**Solution:** Resize your terminal window to at least 100 columns width, or maximize the window.

### FFplay not found
**Issue:** `ffplay: command not found` or similar error.

**Solution:** Install FFmpeg:
```bash
# macOS
brew install ffmpeg

# Linux
sudo apt-get install ffmpeg
```

### Server connection failed
**Issue:** Cannot connect to the server at `localhost:8080`.

**Solution:** 
- The client should automatically start the server. If it doesn't, try starting it manually:
  ```bash
  ./target/release/radio_server &
  ```
- If port 8080 is in use, specify a different port:
  ```bash
  ./target/release/radio_client -a localhost:8081 search jazz
  ```

### No stations found
**Issue:** Search returns no results.

**Solution:**
- Check your internet connection
- Try broadening your search (use fewer filters)
- The radio-browser API might be temporarily unavailable

## Project Structure

```
Radiokl/
├── src/
│   ├── bin/
│   │   ├── radio_client/    # Client application
│   │   │   ├── main.rs       # Entry point and CLI parsing
│   │   │   ├── rclient.rs    # Client logic
│   │   │   ├── chooser.rs    # TUI interface
│   │   │   └── server_initializer.rs  # Server management
│   │   └── radio_server/    # Server application
│   ├── lib.rs               # Shared data structures
│   └── utils.rs             # Shared utilities
├── Cargo.toml
└── README.md
```

## Contributing

Contributions are welcome! Here's how you can help:

1. **Report bugs** by opening an issue
2. **Suggest features** or improvements
3. **Submit pull requests** with bug fixes or new features

When contributing code:
- Follow Rust coding conventions
- Test your changes thoroughly
- Update documentation as needed

## License

This project is open source. Please add appropriate license information.

## Acknowledgments

- [radio-browser API](https://api.radio-browser.info/) for providing the extensive radio station database
- [FFmpeg](https://ffmpeg.org/) for audio streaming capabilities
- The Rust community for excellent async runtime libraries (Tokio, async-std)

## Future Enhancements

- [ ] Cross-platform support (Windows)
- [ ] Multiple favorite lists/playlists
- [ ] Station metadata display (now playing info)
- [ ] Recording functionality
- [ ] Configuration file support

---

**Note:** This software is in active development. The terminal window must be wide enough for the TUI to function properly.
