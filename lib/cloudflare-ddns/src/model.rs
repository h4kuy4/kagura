use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub success: bool,
    pub result: Vec<Record>,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub content: String,
    pub name: String,
    pub id: String,
}
