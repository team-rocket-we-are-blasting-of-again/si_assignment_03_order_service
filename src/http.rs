use reqwest::{Error, Url};
use serde::de::DeserializeOwned;

pub async fn make_request<T: DeserializeOwned>(url: Url) -> Result<T, Error> {
    reqwest::get(url).await?.json::<T>().await
}
