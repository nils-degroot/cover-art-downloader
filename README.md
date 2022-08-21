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
    cover-art-fetcher [OPTIONS] <ARTIST> <RELEASE>

ARGS:
    <ARTIST>     Artist to search for
    <RELEASE>    Release name to search for

OPTIONS:
    -f, --filename <FILENAME>
            Filename to output to [default: cover.jpg]

    -h, --help
            Print help information

    -t, --target-directory <TARGET_DIRECTORY>
            File to push to output to, defaults to the current working directory
```
