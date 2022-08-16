use crate::data::{CLIENT, COVERARCHIVE_BASE_URL, DEFAULT_HEADERS};
use bytes::Bytes;
use reqwest::StatusCode;

use super::{RequestFailures, RequestResult};

pub fn cover<S: ToString>(release_group_id: S) -> RequestResult<Option<Bytes>> {
    let url = format!(
        "{}/release-group/{}/front-500",
        COVERARCHIVE_BASE_URL,
        release_group_id.to_string()
    );

    let response = CLIENT
        .get(url)
        .headers(DEFAULT_HEADERS.clone())
        .send()
        .map_err(|_| RequestFailures::ErrorSendingRequest)?;

    match response.status() {
        StatusCode::OK => {}
        StatusCode::NOT_FOUND => return Ok(None),
        _ => Err(RequestFailures::ErrorSendingRequest)?,
    };

    Ok(Some(response.bytes().unwrap()))
}
