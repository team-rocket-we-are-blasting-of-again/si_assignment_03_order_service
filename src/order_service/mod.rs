pub mod order_service {

    use crate::{
        grpc::{order, student_has_funds},
        students::BoughtBookReply,
        types::Title,
    };
    use std::error::Error;

    pub async fn can_order_book(
        book: &Title,
        student_id: i32,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let student_response = student_has_funds(book.price, student_id).await;

        match student_response {
            Ok(has_funds) => return Ok(book.onStock > 0 && has_funds),
            Err(e) => return Err(e),
        }
    }

    pub async fn make_order(
        book: &Title,
        student_id: i32,
    ) -> Result<BoughtBookReply, Box<dyn Error + Send + Sync>> {
        order(book.id, student_id, book.price).await
    }
}
