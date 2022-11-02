use serde::Deserialize;

#[derive(Deserialize)]
pub struct Title {
    id: i32,
    title: String,
    authorFirstName: String,
    authorLastName: String,
    edition: i32,
    year: i32,
    price: f64,
    pub onStock: i32  
}

#[derive(Deserialize)]
pub struct OrderRequest {
    student_id: i32,
    title_id: i32
}