use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct School {
    #[serde(rename="kraOrgNm")]
    pub name: String,
    #[serde(rename="engOrgNm")]
    pub english_name: String,
    #[serde(rename="orgCode")]
    pub code: String,
    #[serde(rename="addres")]
    pub address: String
}

#[derive(Deserialize, Debug)]
pub struct University {
    #[serde(rename="kraOrgNm")]
    pub name: String,
    #[serde(rename="engOrgNm")]
    pub english_name: String,
    #[serde(rename="orgCode")]
    pub code: String,
    #[serde(rename="addres")]
    pub address: String
}

#[derive(Deserialize, Debug)]
pub struct Office {
    #[serde(rename="kraOrgNm")]
    pub name: String,
    #[serde(rename="engOrgNm")]
    pub english_name: Option<String>,
    #[serde(rename="orgCode")]
    pub code: String,
    #[serde(rename="addres")]
    pub address: String
}