use serde::{de::DeserializeOwned};
use reqwest::Error;

pub async fn make_request<T: DeserializeOwned>(url: String) -> Result<T, Error> {
    reqwest::get(url)
    .await?
    .json::<T>()
    .await
}