use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Result,
};
use reqwest::blocking::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::io::{stdout, Write};

#[derive(Debug, Serialize, Deserialize)]
struct ArtistReponse {
    artists: Vec<Artist>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Artist {
    id: String,
    #[serde(rename = "sort-name")]
    name: String,
    area: Area,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Area {
    name: String,
}

fn client() -> Client {
    ClientBuilder::new().build().unwrap()
}

fn artist<S: ToString>(name: S) -> Vec<Artist> {
    let response = client()
        .get(format!(
            "http://musicbrainz.org/ws/2/artist/?query=artist:{}",
            name.to_string()
        ))
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:103.0) Gecko/20100101 Firefox/103.0",
        )
        .header("Accept", "application/json")
        .send()
        .unwrap();

    response.json::<ArtistReponse>().unwrap().artists
}

fn read_char() -> Result<KeyTypes> {
    loop {
        break match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => Ok(KeyTypes::ArrowUp),
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => Ok(KeyTypes::ArrowDown),
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => Ok(KeyTypes::Char(c)),
            _ => continue,
        };
    }
}

enum KeyTypes {
    ArrowUp,
    ArrowDown,
    Char(char),
}

fn main() -> Result<()> {
    let mut w = stdout();
    execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    let mut cursor_pos = 0;
    let artists = artist("Rammstein");

    loop {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(cursor_pos, 0)
        )?;

        for artist in artists.clone() {
            queue!(
                w,
                style::Print(format!("{} - {}", artist.name, artist.area.name)),
                cursor::MoveToNextLine(1)
            )?;
        }

        w.flush()?;

        match read_char()? {
            KeyTypes::ArrowUp | KeyTypes::Char('k') => {
                cursor_pos -= 1;
            }
            KeyTypes::ArrowDown | KeyTypes::Char('j') => {
                cursor_pos += 1;
            }
            KeyTypes::Char('q') => break,
            _ => {}
        };
    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
}
