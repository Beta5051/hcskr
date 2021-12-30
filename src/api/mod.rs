pub mod institution;
pub mod user;

use reqwest::{Client, Error, Response};
use serde::Serialize;

async fn get<T: Serialize + ?Sized>(url: String, query: &T) -> Result<Response, Error> {
    let client = Client::new();
    client.get(url).query(query).send().await
}

async fn post<T: Serialize + ?Sized>(url: String, json: &T) -> Result<Response, Error> {
    let client = Client::new();
    client.post(url).json(json).send().await
}