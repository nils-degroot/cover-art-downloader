use crate::data::{CLIENT, DEFAULT_HEADERS, MUSICBRAINZ_BASE_URL};
use serde::{Deserialize, Serialize};

use super::{RequestFailures, RequestResult};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ReleaseGroupReponse {
    release_groups: Vec<ReleaseGroup>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ReleaseGroup {
    id: String,
    title: String,
    #[serde(rename = "primary-type")]
    release_type: String,
}

impl ReleaseGroup {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn release_type(&self) -> String {
        self.release_type.clone()
    }
}

pub fn release_groups<S: ToString>(artist_id: S, name: S) -> RequestResult<Vec<ReleaseGroup>> {
    let url = format!(
        "{}/release-group/?query=arid:{}%20AND%20release:{}",
        MUSICBRAINZ_BASE_URL,
        artist_id.to_string(),
        name.to_string()
    );

    let response = CLIENT
        .get(url)
        .headers(DEFAULT_HEADERS.clone())
        .send()
        .map_err(|_| RequestFailures::ErrorSendingRequest)?;

    Ok(response
        .json::<ReleaseGroupReponse>()
        .map_err(|_| RequestFailures::FailedToSerialize)?
        .release_groups)
}
