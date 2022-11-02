use crate::students::book_purchase_service_client::BookPurchaseServiceClient;
use crate::students::StudentInfo;

pub async fn make_request(price: f64, student_id: i32) -> Result<bool, Box<dyn std::error::Error>> {
    let mut client = BookPurchaseServiceClient::connect("url").await?;
    let request = tonic::Request::new(
        StudentInfo {
            price,
            student_id: student_id as i64
        }
    );
    let response = client.student_has_funds(request).await?;

    return Ok(*response.get_ref());
}