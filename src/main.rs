use clap::{Parser, Subcommand};
use data::{artist::artists, release_group::release_groups};
use ext::*;
use std::{
    env,
    fs::File,
    io::{stdin, stdout, Write},
    path::PathBuf,
};

mod data;
mod ext;

lazy_static::lazy_static! {
    static ref DEFAULT_DIRECTORY: PathBuf = env::current_dir().expect("Failed to get current working directory");
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
        /// Filename to output to
        #[clap(short, long, default_value_t = String::from("cover.jpg"))]
        filename: String,
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
                target_path.push(filename);
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
    let artist = handle_multiple(
        artists(context.artist).expect("Request error occured while fetching artist"),
        "artist".to_string(),
    )
    .expect("Failed to find artist");

    let release = handle_multiple(
        release_groups(artist.id(), context.release)
            .expect("Request eror occured while fetching release group"),
        "release group".to_string(),
    )
    .expect("Failed to find a release group");

    let cover = data::cover::cover(release.id())
        .expect("Request error occured while fetching cover")
        .expect("Failed to get cover");

    File::create(context.target_directory)
        .expect("Failed to create output file")
        .write_all(&cover)
        .expect("Failed to write to output file");
}

fn handle_multiple<T>(options: Vec<T>, type_name: String) -> Option<T>
where
    T: Sized + Clone + Id + ReadableForm,
{
    match &options[..] {
        [] => None,
        [option] => Some(option).cloned(),
        _ => {
            println!("Select a {}", type_name);
            Some(select_item(options))
        }
    }
}

fn select_item<T>(options: Vec<T>) -> T
where
    T: Clone + ReadableForm,
{
    let options = options.iter().enumerate().collect::<Vec<_>>();

    loop {
        for (index, option) in &options {
            println!("{} - {}", index + 1, option.readable_from());
        }

        print!("\n >> ");
        stdout().flush().unwrap();
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
            break options.get(selected).unwrap().1.clone();
        }

        println!("Invalid index selected\n");
    }
}
