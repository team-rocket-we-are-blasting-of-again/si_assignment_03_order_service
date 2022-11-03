use rocket::response::status;
use rocket::serde::json::Json;

use crate::{types::OrderRequest, order_service::order_service::can_order_book, order_service::order_service:: make_order, book_service::book_service::get_book};

#[post("/orderBook", data = "<order>")]
pub async fn order_book(order: Json<OrderRequest>) -> Option<status::NoContent> {
    let book = get_book(order.title_id).await.ok()?;
    let can_order_book = can_order_book(&book, order.student_id).await.ok()?;

    if can_order_book {
        let _order_req = make_order(&book, order.student_id).await.ok();
        return Some(status::NoContent);
    }

    Some(status::NoContent)
}