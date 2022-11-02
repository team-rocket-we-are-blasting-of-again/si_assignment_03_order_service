use rocket::response::status;
use rocket_contrib::json::Json;

use crate::types::OrderRequest;

#[post("/orderBook", data = "<order>")]
pub fn order_book(order: Json<OrderRequest>) -> status::NoContent {
    status::NoContent
}