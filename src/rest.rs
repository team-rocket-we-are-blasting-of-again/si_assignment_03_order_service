use rocket::response::{status, Debug};
use rocket::serde::json::Json;
use crate::{types::OrderRequest, order_service::order_service::can_order_book, order_service::order_service:: make_order, book_service::book_service::get_book};

#[post("/orderBook", data = "<order>")]
pub async fn order_book(order: Json<OrderRequest>) -> Result<(), Debug<Box<dyn std::error::Error>>> {
    let book_response = get_book(order.title_id).await;

    let book = match book_response {
        Ok(book) => book,
        Err(e) => panic!("{}", e),
    };

    let can_order_response = can_order_book(&book, order.student_id).await;
    
    let can_order = match can_order_response {
        Ok(can_order) => can_order,
        Err(e) => panic!("{}", e),
    };

    if can_order {
        let _order = make_order(&book, order.student_id);
    }

    Ok(())
}