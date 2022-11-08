#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use dotenv::dotenv;

mod book_service;
mod grpc;
mod http;
mod order_service;
mod rest;
mod types;
pub mod students {
    tonic::include_proto!("com.teamrocket");
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let _rocket = rocket::build()
        .mount("/orders", routes![rest::order_book,])
        .launch()
        .await?;

    Ok(())
}
