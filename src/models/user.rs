use serde::{Deserialize};
use crate::utils;

#[derive(Deserialize, Debug)]
pub struct UsersInfo {
    #[serde(rename="userName")]
    pub main_user_name: String,
    pub token: String,
    #[serde(rename="stdntYn", deserialize_with="utils::deserialize_string_to_bool")]
    pub is_student: bool,
    #[serde(rename="admnYn", deserialize_with="utils::deserialize_string_to_bool")]
    pub is_admin: bool,
    #[serde(rename="pInfAgrmYn", deserialize_with="utils::deserialize_string_to_bool")]
    pub is_agree: bool
}