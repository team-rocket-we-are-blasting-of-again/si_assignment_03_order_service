use rocket::response::status;
use rocket_contrib::json::Json;

use crate::{types::OrderRequest, order_service::order_service::can_order_book};

#[post("/orderBook", data = "<order>")]
pub async fn order_book(order: Json<OrderRequest>) -> Option<status::NoContent> {
    can_order_book(order.title_id, order.student_id).await.ok();
    Some(status::NoContent)
}