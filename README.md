# Cover art fetcher

A CLI for downloading cover art of music from Musicbrainz.

## Installation

Installation of the project requires cargo to be installed

```sh
git clone git://git.peeko.nl/cover-art-fetcher.git
cd cover-art-fetcher
cargo b --release
sudo ln -s $PWD/target/release/cover-art-fetcher /usr/bin
```

## Usage

```
cover-art-fetcher 

USAGE:
    cover-art-fetcher <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    fetch    Fetches a release cover based on the parameters
    help     Print this message or the help of the given subcommand(s)
```
