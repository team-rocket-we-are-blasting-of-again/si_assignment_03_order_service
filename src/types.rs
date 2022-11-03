use serde::Deserialize;

#[derive(Deserialize)]
pub struct Title {
    pub id: i32,
    title: String,
    authorFirstName: String,
    authorLastName: String,
    edition: i32,
    year: i32,
    pub price: f64,
    pub onStock: i32  
}

#[derive(Deserialize)]
pub struct OrderRequest {
    pub student_id: i32,
    pub title_id: i32
}