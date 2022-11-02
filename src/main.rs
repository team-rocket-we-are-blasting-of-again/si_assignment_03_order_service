#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod rest;
mod http;
mod order_service;
mod types;
mod grpc;
pub mod students {
    tonic::include_proto!("com.teamrocket");
}

#[tokio::main]
async fn main() {
    rocket::ignite().mount("/", routes![
        rest::order_book,
    ]).launch();
}
