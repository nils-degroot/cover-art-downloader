use super::{RequestFailures, RequestResult};
use crate::data::{CLIENT, DEFAULT_HEADERS, MUSICBRAINZ_BASE_URL};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ArtistReponse {
    artists: Vec<Artist>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Area {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Artist {
    id: String,
    #[serde(rename = "sort-name")]
    name: String,
    area: Option<Area>,
}

impl Artist {
    pub fn id(&self) -> String {
        self.id.clone()
    }
}

pub fn artists<S: ToString>(name: S) -> RequestResult<Vec<Artist>> {
    let url = format!(
        "{}/artist/?query=artist:{}",
        MUSICBRAINZ_BASE_URL,
        name.to_string()
    );

    let response = CLIENT
        .get(url)
        .headers(DEFAULT_HEADERS.clone())
        .send()
        .map_err(|_| RequestFailures::ErrorSendingRequest)?;

    Ok(response
        .json::<ArtistReponse>()
        .map_err(|_| RequestFailures::FailedToSerialize)?
        .artists)
}

pub fn artist<S: ToString>(name: S) -> RequestResult<Option<Artist>> {
    Ok(artists(name)?.first().cloned())
}
