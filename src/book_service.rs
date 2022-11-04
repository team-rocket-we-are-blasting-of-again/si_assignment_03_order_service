pub mod book_service {
    use std::env;
    use reqwest::Url;

    use crate::{http::make_request, types::Title};

    pub async fn get_book(book_id: i32) -> Result<Title, reqwest::Error>{
        let base_url = env::var("LIBRARY_SERVICE_URL").unwrap();
        let url = Url::parse(&(base_url + "/titles/" + &book_id.to_string()));
        match url {
            Ok(url) => make_request::<Title>(url).await,
            Err(_) => todo!(),
        }
        
    }
}