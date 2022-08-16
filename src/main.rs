use clap::{Parser, Subcommand};
use data::{artist::artists, release_group::release_groups};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{stdin, Write},
    path::PathBuf,
};

mod data;

lazy_static::lazy_static! {
    static ref DEFAULT_DIRECTORY: PathBuf = env::current_dir().expect("Failed to get current working directory");
}

const DEFAULT_FILENAME: &'_ str = "cover.jpg";

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
        #[clap(short, long)]
        filename: Option<String>,
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
            filename,
        } => fetch(FetchContext {
            artist,
            release,
            target_directory: {
                let mut target_path = target_directory.unwrap_or_else(|| DEFAULT_DIRECTORY.clone());
                target_path.push(filename.unwrap_or(DEFAULT_FILENAME.to_string()));
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
    let artists = artists(context.artist).unwrap();
    let artist = match artists.len() {
        0 => panic!("Failed to find artist"),
        1 => artists.first().unwrap(),
        _ => {
            let artist_map = artists
                .iter()
                .map(|artist| (artist.id(), artist.name()))
                .collect::<HashMap<_, _>>();

            println!("Select a artist");
            let selected = select_item(artist_map);
            artists
                .iter()
                .find(|artist| artist.id() == selected)
                .unwrap()
        }
    };

    let release_groups = release_groups(artist.id(), context.release).unwrap();
    let release_group = match release_groups.len() {
        0 => panic!("Failed to find release group"),
        1 => release_groups.first().unwrap(),
        _ => {
            let release_group_map = release_groups
                .iter()
                .map(|group| {
                    (
                        group.id(),
                        format!("{} - {}", group.release_type(), group.title()),
                    )
                })
                .collect::<HashMap<_, _>>();

            println!("Select a release");
            let selected = select_item(release_group_map);
            release_groups
                .iter()
                .find(|group| group.id() == selected)
                .unwrap()
        }
    };

    let cover = data::cover::cover(release_group.id())
        .expect("Request error occured while fetching cover")
        .expect("Failed to get cover");

    File::create(context.target_directory)
        .expect("Failed to create output file")
        .write_all(&cover)
        .expect("Failed to write to output file");
}

fn select_item(options: HashMap<String, String>) -> String {
    let options = options.iter().enumerate().collect::<Vec<_>>();

    loop {
        for (index, (_, text)) in &options {
            println!("{} - {}", index + 1, text);
        }

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        let selected = match buffer.trim().parse::<usize>() {
            Ok(i) => i - 1,
            Err(_) => {
                println!("Invalid input\n");
                continue;
            }
        };

        if (0..options.len()).contains(&selected) {
            break options.get(selected).unwrap().1 .0.clone();
        }

        println!("Invalid index selected\n");
    }
}
