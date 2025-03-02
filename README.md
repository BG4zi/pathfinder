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
    -c, --status_codes <status_codes>
            only prints the specified status codes codes should be listed like 200,400,500 [default:
            NO_CODE]

        --debug <debug-mode>
            [default: 1]

    -h, --help
            Print help information

    -o, --output <output>
            Saves the log into the file that's given

    -u, --url <url>
            The base URL

    -V, --version
            Print version information

    -w, --wordlist <wordlist>
            Path to wordlist
```

## Features
- Simple getting the paths from wordlist of the website with status codes
- Specifiable Status Codes to get
- Save the log
- Fast, uses Coroutines
- Colorful
