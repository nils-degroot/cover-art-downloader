use clap::{Parser, Subcommand};
use std::{fs::File, io::Write, path::PathBuf};

mod data;

lazy_static::lazy_static! {
    static ref DEFAULT_DIRECTORY: PathBuf = std::env::current_dir().expect("Failed to get current working directory");
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Fetches a release cover based on the parameters
    Fetch {
        /// Artist to search for
        artist: String,
        /// Release name to search for
        release: String,
        /// File to push to output to, defaults to the current working directory
        #[clap(short, long)]
        target_directory: Option<PathBuf>,
    },
}

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    match Cli::parse().command {
        Commands::Fetch {
            artist,
            release,
            target_directory,
        } => fetch(FetchContext {
            artist,
            release,
            target_directory: {
                let mut target_path = target_directory.unwrap_or_else(|| DEFAULT_DIRECTORY.clone());
                target_path.push("cover.jpg");
                target_path
            },
        }),
    }
}

struct FetchContext {
    artist: String,
    release: String,
    target_directory: PathBuf,
}

fn fetch(context: FetchContext) {
    let artist = data::artist::artist(context.artist)
        .expect("Request error occured while fetching artist")
        .expect("Failed to find artist");

    let release_group = data::release_group::release_group(artist.id(), context.release)
        .expect("Request error occured while fetching release group")
        .expect("Failed to find release group");

    let cover = data::cover::cover(release_group.id())
        .expect("Request error occured while fetching cover")
        .expect("Failed to get cover");

    let mut file = File::create(context.target_directory).expect("Failed to create output file");
    file.write_all(&cover)
        .expect("Failed to write to output file");
}
