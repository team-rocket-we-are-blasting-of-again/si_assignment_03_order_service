pub mod order_service {
    
    use std::env;

    use crate::{http::make_request, types::Title, grpc::student_has_funds};

    pub async fn can_order_book(book_id: i32, student_id: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let url = env::var("LIBRARY_SERVICE_URL").unwrap();
        let title_response = make_request::<Title>(url + "/titles/" + &book_id.to_string()).await;
        

        match title_response {
            Ok(title) => {
                let student_response = student_has_funds(title.price, student_id).await;

                match student_response {
                    Ok(has_funds) => return Ok(title.onStock > 0 && has_funds),
                    Err(e) => return Err(e),
                }
            },
            Err(e) => {
                println!("{}", e);
                return Err(Box::new(e));
            },
        }
    }
}