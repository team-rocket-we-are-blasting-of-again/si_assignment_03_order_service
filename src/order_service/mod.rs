pub mod order_service {
    use crate::{http::make_request, types::Title};

    async fn check_book_availability(student_id: i32) -> bool {
        let response = make_request::<Title>(String::from("localhost:8080/titles") + &student_id.to_string()).await;

        match response {
            Ok(title) => {
                return !title.onStock <= 0;
            },
            Err(e) => {
                println!("{}", e);
                return false;
            },
        }
    }
}