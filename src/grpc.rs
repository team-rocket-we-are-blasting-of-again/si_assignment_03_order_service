use std::env;
use std::error::Error;

use crate::students::{
    book_purchase_service_client::BookPurchaseServiceClient, BookToBuy, BoughtBookReply,
    StudentInfo,
};

pub async fn student_has_funds(
    price: f64,
    student_id: i32,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let url = env::var("STUDENT_SERVICE_URL").unwrap();
    let mut client = BookPurchaseServiceClient::connect(url).await?;
    let request = tonic::Request::new(StudentInfo {
        price,
        student_id: student_id.into(),
    });
    let response = client.student_has_funds(request).await?;

    return Ok(response.into_inner());
}

pub async fn order(
    book_id: i32,
    student_id: i32,
    price: f64,
) -> Result<BoughtBookReply, Box<dyn Error + Send + Sync>> {
    let url = env::var("STUDENT_SERVICE_URL").unwrap();
    let mut client = BookPurchaseServiceClient::connect(url).await?;
    let request = tonic::Request::new(BookToBuy {
        book_id: book_id.into(),
        student_id: student_id.into(),
        price,
    });
    let response = client.purchase_book(request).await?;

    return Ok(response.into_inner());
}
