use crate::api::*;
use crate::encrypt::encrypt_text;
use crate::data;
use std::fmt;
use std::error::Error;
use std::collections::HashMap;
use serde::Deserialize;

pub fn self_check(name: &str, birthday: &str, area: &str, school_name: &str, level: &str, password: &str, user_name: &str) -> Result<HcsResult, Box<dyn Error>> {
    Hcs::new().self_check(name, birthday, area, school_name, level, password, user_name)
}

pub struct Hcs { api: API }

impl Hcs {
    pub fn new() -> Self { Self { api: API::new() } }

    pub fn self_check(&self, name: &str, birthday: &str, area: &str, school_name: &str, level: &str, password: &str, user_name: &str) -> Result<HcsResult, Box<dyn Error>> {
        let area_data = data::get_area_data(area);
        if area_data.is_none() {
            return Err(Box::new(HcsError::new("해당 지역이 존재하지 않습니다.")));
        }
        let (lctn_sc_code, url_code) = area_data.unwrap();
        let level_data = data::get_level_data(level);
        if level_data.is_none() {
            return Err(Box::new(HcsError::new("해당 학교 유형이 존재 하지 않습니다.")));
        }
        let schul_crse_code = level_data.unwrap();
        let schools = self.search_school(lctn_sc_code, schul_crse_code, school_name)?;
        if schools.len() == 0 {
            return Err(Box::new(HcsError::new("해당 학교를 찾지 못했습니다.")));
        }
        let org_code = schools[0].org_code.as_str();
        let user = self.find_user(url_code, org_code, name, birthday)?;
        let mut token = user.token;
        if !self.has_password(url_code, token.as_str())? {
            return Err(Box::new(HcsError::new("비밀번호가 존재하지 않습니다.")));
        }
        token = self.validate_password(url_code, token.as_str(), password)?;
        let user_info = self.get_user_info(url_code, token.as_str(), org_code)?;
        token = user_info.token;
        self.register_servey(url_code, token.as_str(), false, false, false, user_name)
    }

    pub fn search_school(&self, lctn_sc_code: &str, schul_crse_sc_code: u8, org_name: &str) -> Result<Vec<School>, Box<dyn Error>>{
        let mut data = HashMap::new();
        let schul_crse_sc_code_string = schul_crse_sc_code.to_string();
        data.insert("lctnScCode", lctn_sc_code);
        data.insert("schulCrseScCode", schul_crse_sc_code_string.as_str());
        data.insert("orgName", org_name);
        let result: SchoolSearchResponse = self.api.call(APIMethod::GET, "", "", "v2/searchSchool", Some(data))?;
        Ok(result.schul_list)
    }

    pub fn find_user(&self, url_code: &str, org_code: &str, name: &str, birthday: &str) -> Result<User, Box<dyn Error>> {
        let name_encrypt = encrypt_text(name)?;
        let birthday_encrypt = encrypt_text(birthday)?;
        let mut data = HashMap::new();
        data.insert("orgCode", org_code);
        data.insert("name", name_encrypt.as_str());
        data.insert("birthday", birthday_encrypt.as_str());
        self.api.call(APIMethod::POST, url_code, "", "v2/findUser", Some(data))
    }

    pub fn has_password(&self, url_code: &str, token: &str) -> Result<bool, Box<dyn Error>> {
        self.api.call(APIMethod::POST, url_code, token, "v2/hasPassword", None)
    }

    pub fn validate_password(&self, url_code: &str, token: &str, password: &str) -> Result<String, Box<dyn Error>> {
        let password_encrypt = encrypt_text(password)?;
        let mut data = HashMap::new();
        data.insert("password", password_encrypt.as_str());
        self.api.call(APIMethod::POST, url_code, token, "v2/validatePassword", Some(data))
    }

    pub fn select_user_group(&self, url_code: &str, token: &str) -> Result<Vec<UserGroup>, Box<dyn Error>> {
        self.api.call(APIMethod::POST, url_code, token, "v2/selectUserGroup", None)
    }

    pub fn get_user_info(&self, url_code: &str, token: &str, org_code: &str) -> Result<UserInfo, Box<dyn Error>> {
        let mut data = HashMap::new();
        data.insert("orgCode", org_code);
        self.api.call(APIMethod::POST, url_code, token, "v2/getUserInfo", Some(data))
    }

    pub fn register_servey(&self, url_code: &str, token: &str, rspns01: bool, rspns02: bool, rspns03: bool, user_name: &str) -> Result<HcsResult, Box<dyn Error>> {
        let mut data = HashMap::new();
        let rspns01_string = (!rspns01 as u8).to_string();
        let rspns02_string = (!rspns02 as u8).to_string();
        data.insert("rspns01", rspns01_string.as_str());
        data.insert("rspns02", rspns02_string.as_str());
        data.insert("rspns03", if rspns03 {
            "N"
        } else {
            "Y"
        });
        data.insert("upperToken", token);
        data.insert("upperUserNameEncpt", user_name);
        self.api.call(APIMethod::POST, url_code, token, "registerServey", Some(data))
    }
}

#[derive(Debug)]
pub struct HcsError { details: String }

impl HcsError {
    fn new(details: &str) -> Self { Self { details: details.to_string() } }
}

impl fmt::Display for HcsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "HcsError: {}", self.details) }
}

impl Error for HcsError {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SchoolSearchResponse { schul_list: Vec<School> }

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct School {
    pub org_code: String,
    pub kra_org_nm: String,
    pub eng_org_nm: String,
    pub instt_clsf_code: String,
    pub lctn_sc_code: String,
    pub lctn_sc_nm: String,
    pub sig_code: String,
    pub ju_org_code: String,
    pub schul_knd_sc_code: String,
    pub org_abrv_nm01: String,
    pub org_abrv_nm02: String,
    pub org_uon: String,
    pub updid: String,
    pub mdfc_dtm: String,
    pub atpt_ofcdc_conct_url: String,
    pub addres: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub org_name: String,
    pub admn_yn: String,
    pub atpt_ofcdc_conct_url: String,
    pub mngr_class_yn: String,
    pub p_inf_agrm_yn: String,
    pub user_name: String,
    pub stdnt_yn: String,
    pub token: String,
    pub mngr_dept_yn: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGroup {
    pub org_code: String,
    pub org_name: String,
    #[serde(rename = "userPNo")]
    pub user_pno: String,
    pub user_name_encpt: String,
    pub stdnt_yn: String,
    pub mngr_yn: String,
    pub schul_crse_sc_code: String,
    pub lctn_sc_code: String,
    pub token: String,
    pub atpt_ofcdc_conct_url: String,
    pub wrong_pass_cnt: i64,
    pub other_yn: String,
    pub rn: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub org_code: String,
    pub org_name: String,
    #[serde(rename = "userPNo")]
    pub user_pno: String,
    pub user_name_encpt: String,
    pub user_name: String,
    pub stdnt_yn: String,
    pub mngr_class_yn: String,
    pub mngr_dept_yn: String,
    pub schul_crse_sc_code: String,
    pub lctn_sc_code: String,
    pub instt_clsf_code: String,
    pub device_uuid: String,
    pub token: String,
    pub atpt_ofcdc_conct_url: String,
    pub p_inf_agrm_yn: String,
    pub admn_yn: String,
    pub lock_yn: String,
    pub wrong_pass_cnt: i64,
    pub ext_survey_count: i64,
    pub ext_survey_remain_count: i64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HcsResult {
    pub register_dtm: String,
    pub inve_ymd: String,
}