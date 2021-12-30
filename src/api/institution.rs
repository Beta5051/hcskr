use std::collections::HashMap;
use reqwest::Error;
use serde::Deserialize;
use crate::models::{School, University, Office, Area, Level};
use crate::utils;

#[derive(Deserialize)]
struct Response<T> {
    schulList: Vec<T>
}

pub async fn get_school_data(area: Area, level: Level, name: &str) -> Result<Vec<School>, Error> {
    let response = super::get(utils::get_endpoint(None, "searchSchool"), &[("lctnScCode", area.get_code()), ("schulCrseScCode", level.get_code()), ("orgName", name), ("loginType", "school")])
        .await?.json::<Response<School>>().await?;
    Ok(response.schulList)
}

pub async fn get_university_data(name: &str) -> Result<Vec<University>, Error> {
    let response = super::get(utils::get_endpoint(None, "searchSchool"), &[("orgName", name), ("loginType", "univ")])
        .await?.json::<Response<University>>().await?;
    Ok(response.schulList)
}

pub async fn get_office_data(name: &str) -> Result<Vec<Office>, Error> {
    let response = super::get(utils::get_endpoint(None, "searchSchool"), &[("orgName", name), ("loginType", "office")])
        .await?.json::<Response<Office>>().await?;
    Ok(response.schulList)
}