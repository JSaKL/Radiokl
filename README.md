# Radiokl

Radiokl is a simple command line application (for Linux and Mac) for searching and playing radio stations (with streaming links).

## Description

The software consists of two parts: radio_client, which acts as a front end for this and radio_server (those are also names of the executables).
The server part of this software uses radio-browser-api (https://api.radio-browser.info/) directly without 
any third party libraries to get a list of all available servers (by doing a DNS-lookup of 'all.api.radio-browser.info' etc).
Then it uses the selected server for searching radio stations in the internet.
For audio streaming this sorfware uses FFplay media player (using the FFmpeg libraries and the SDL library).

## Getting Started

### Dependencies

Notice that this works only on Linux and Mac. 

This software uses FFplay so FFmpeg has to be installed on same the computer for audio streaming. 

### Installing

* Clone or download this software
* Build it with "cargo build --release" in the project root directory

### Executing program

Notice that because this software is still very much in-progress the terminal window must be wide enough for the TUI part of this software to execute smoothly.

* Execute the sofware in the target directory (/release/target)
* The client (radio_client) starts the server automatically if it's not already running.
* The server can be started also manually first.
```
jsa@jsa-MacBookPro:~/devs/radiokl/target/release$ ./radio_client -h
Usage: radio_client [OPTIONS] <COMMAND>

Commands:
  search       Search radio station
  favs         Get radio station from saved favorites
  stop         Stop the radio station stream
  stop-server  Stop the radio stream server
  help         Print this message or the help of the given subcommand(s)

Options:
  -c, --country <COUNTRY>    Country of the radio station to search
  -l, --language <LANGUAGE>  Language of the radio station to search
  -a, --addr <ADDR>          Stream connection address [default: localhost:8080]
  -h, --help                 Print help
  -V, --version              Print version

jsa@jsa-MacBookPro:~/devs/radiokl/target/release$ ./radio_client search jazz

Here's the list (partly) view output for the above command:

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
