use std::fmt;
use std::error::Error;
use std::collections::HashMap;
use reqwest::Method;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde_json::Value;

fn get_endpoint(url_code: &str, path: &str) -> String { format!("https://{}hcs.eduro.go.kr/{}", url_code, path) }

#[derive(PartialEq)]
pub enum APIMethod { GET, POST }

impl APIMethod {
    fn to_method(&self) -> Method {
        if *self == Self::GET {
            return Method::GET;
        }
        Method::POST
    }
}

pub struct API { client: Client }

impl API {
    pub fn new() -> Self { Self { client: Client::new() } }

    pub fn call<T: DeserializeOwned>(&self, method: APIMethod, url_code: &str, token: &str, path: &str, data: Option<HashMap<&str, &str>>) -> Result<T, Box<dyn Error>> {
        let mut request = self.client.request(method.to_method(), get_endpoint(url_code, path));
        if method == APIMethod::POST {
            request = request.header("Content-Type", "application/json");
        }
        if !token.is_empty() {
            request = request.header("Authorization", token);
        }
        if let Some(data) = data {
            if method == APIMethod::GET {
                let mut query = Vec::new();
                for (&key, &value) in &data {
                    query.push((key, value));
                }
                request = request.query(query.as_slice());
            } else {
                request = request.json(&data);
            }
        }
        let response = request.send()?;
        let text = response.text()?;
        let text_str = text.as_str();
        if let Ok(json_data) = serde_json::from_str::<Value>(text_str) {
            if json_data.get("isError").is_some() {
                return Err(Box::new(APIError::new(text_str)));
            }
            return Ok(serde_json::from_value(json_data)?);
        }
        Err(Box::new(APIError::new(text_str)))
    }
}

#[derive(Debug)]
pub struct APIError { details: String }

impl APIError {
    fn new(details: &str) -> Self { Self { details: details.to_string() } }
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "APIError: {}", self.details) }
}

impl Error for APIError {}