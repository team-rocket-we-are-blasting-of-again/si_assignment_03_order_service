use serde::{de::DeserializeOwned};
use reqwest::{Error, Url};

pub async fn make_request<T: DeserializeOwned>(url: Url) -> Result<T, Error> {
    reqwest::get(url)
    .await?
    .json::<T>()
    .await
}