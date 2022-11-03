#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use dotenv::dotenv;

mod rest;
mod http;
mod order_service;
mod types;
mod grpc;
mod book_service;
pub mod students {
    tonic::include_proto!("com.teamrocket");
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let _rocket = rocket::build().mount("/", routes![
        rest::order_book,
    ]).launch()
    .await?;

    Ok(())
}
