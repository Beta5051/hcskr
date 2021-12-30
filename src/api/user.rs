use std::collections::HashMap;
use std::error::Error;
use crate::models::{Area, LoginType, UsersInfo};
use crate::utils;

pub async fn find(login_type: LoginType, area: Area, code: &str, name: &str, birthday: &str) -> Result<UsersInfo, Box<dyn Error>> {
    let encrypt_name = utils::encrypt_plain_text(name)?;
    let encrypt_birthday = utils::encrypt_plain_text(birthday)?;
    let mut json = HashMap::new();
    json.insert("orgCode", code);
    json.insert("name", encrypt_name.as_str());
    json.insert("birthday", encrypt_birthday.as_str());
    json.insert("loginType", login_type.get_code());
    let response = super::post(utils::get_endpoint(Some(area.get_url_code()), "findUser"), &json)
        .await?.json::<UsersInfo>().await?;
    Ok(response)
}