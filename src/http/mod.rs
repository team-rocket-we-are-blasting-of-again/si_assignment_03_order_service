use serde::Deserialize;

pub async fn make_request<T: for<'a> Deserialize<'a>>(url: String) -> Result<T, reqwest::Error> {
    reqwest::get(url)
    .await?
    .json::<T>()
    .await
}