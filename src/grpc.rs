use std::env;

use crate::students::book_purchase_service_client::BookPurchaseServiceClient;
use crate::students::StudentInfo;

pub async fn student_has_funds(price: f64, student_id: i32) -> Result<bool, Box<dyn std::error::Error>> {
    let url = env::var("STUDENT_SERVICE_URL").unwrap();
    let mut client = BookPurchaseServiceClient::connect(url).await?;
    let request = tonic::Request::new(
        StudentInfo {
            price,
            student_id: student_id as i64
        }
    );
    let response = client.student_has_funds(request).await?;

    return Ok(*response.get_ref());
}