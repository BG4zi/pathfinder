# PathFinder 1.0

A fork of [Dirbuster](https://www.kali.org/tools/dirbuster/) written in Rust.

For now, It is the simplest app it will be developed time by time for new features.

## Installation
```bash
$ git clone https://github.com/BG4zi/pathfinder.git &&
  cd pathfinder
  cargo build
  ./target/debug/pathfinder --version
```

## Usage
```bash
$ pathfinder --url <url> --wordlist <wordlist>	
```

```txt
OPTIONS:
    -h, --help                   Print help information
        --url <url>              The base URL
    -V, --version                Print version information
        --wordlist <wordlist>    Path to wordlist
```

## Features
- Simple getting the paths from wordlist of the website with status codes
- Fast, uses Coroutines
- Colorful
