use crate::data::{CLIENT, DEFAULT_HEADERS, MUSICBRAINZ_BASE_URL};
use serde::{Deserialize, Serialize};

use super::{RequestFailures, RequestResult};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ReleaseGroupReponse {
    release_groups: Vec<ReleaseGroup>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReleaseGroup {
    id: String,
    title: String,
}

impl ReleaseGroup {
    pub fn id(&self) -> String {
        self.id.clone()
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

pub fn release_group<S: ToString>(artist_id: S, name: S) -> RequestResult<Option<ReleaseGroup>> {
    Ok(release_groups(artist_id, name)?.first().cloned())
}
