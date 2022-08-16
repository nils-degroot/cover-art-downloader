use reqwest::{
    blocking::{Client, ClientBuilder},
    header::HeaderMap,
};

pub mod artist;
pub mod cover;
pub mod release_group;

lazy_static::lazy_static! {
    pub static ref DEFAULT_HEADERS: HeaderMap = {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().expect("Failed to parse header"));
        headers.insert(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:103.0) Gecko/20100101 Firefox/103.0"
                .parse()
                .expect("Failed to parse header"),
        );

        headers
    };

    pub static ref CLIENT: Client = ClientBuilder::new().build().expect("Could not initialize TLS backend");
}

pub const MUSICBRAINZ_BASE_URL: &'_ str = "http://musicbrainz.org/ws/2";

pub const COVERARCHIVE_BASE_URL: &'_ str = "http://coverartarchive.org";

type RequestResult<T> = Result<T, RequestFailures>;

#[derive(Debug)]
pub enum RequestFailures {
    ErrorSendingRequest,
    FailedToSerialize,
}
