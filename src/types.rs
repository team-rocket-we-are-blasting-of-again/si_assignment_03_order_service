use serde::Deserialize;

#[derive(Deserialize)]
pub struct Title {
    pub id: i32,
    pub title: String,
    pub authorFirstName: String,
    pub authorLastName: String,
    pub edition: i32,
    pub year: i32,
    pub price: f64,
    pub onStock: i32,
}

#[derive(Deserialize)]
pub struct OrderRequest {
    pub student_id: i32,
    pub title_id: i32,
}
