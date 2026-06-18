use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateAddressRequest {
    pub address: String,
    pub label: String,
    pub phone_number: String,
    pub main: Option<bool>,
}
