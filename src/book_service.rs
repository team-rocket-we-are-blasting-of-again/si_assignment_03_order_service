pub mod book_service {
    use std::env;
    use crate::{http::make_request, types::Title};

    pub async fn get_book(book_id: i32) -> Result<Title, reqwest::Error>{
        let url = env::var("LIBRARY_SERVICE_URL").unwrap();
        make_request::<Title>(url + "/titles/" + &book_id.to_string()).await
    }
}