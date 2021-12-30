use std::convert::TryFrom;

#[derive(Debug)]
pub enum Area {
    Seoul,
    Busan,
    Daegu,
    Incheon,
    Gwangju,
    Daejeon,
    Ulsan,
    Sejong,
    Gyeonggi,
    Gangwon,
    Chungbuk,
    Chungnam,
    Jeonbuk,
    Jeonnam,
    Gyeongbuk,
    Gyeongnam,
    Jeju,
}

impl Area {
    pub fn get_code(&self) -> &'static str {
        match self {
            Self::Seoul => "01",
            Self::Busan => "02",
            Self::Daegu => "03",
            Self::Incheon => "04",
            Self::Gwangju => "05",
            Self::Daejeon => "06",
            Self::Ulsan => "07",
            Self::Sejong => "08",
            Self::Gyeonggi => "10",
            Self::Gangwon => "11",
            Self::Chungbuk => "12",
            Self::Chungnam => "13",
            Self::Jeonbuk => "14",
            Self::Jeonnam => "15",
            Self::Gyeongbuk => "16",
            Self::Gyeongnam => "17",
            Self::Jeju => "18"
        }
    }

    pub fn get_url_code(&self) -> &'static str {
        match self {
            Self::Seoul => "sen",
            Self::Busan => "pen",
            Self::Daegu => "dge",
            Self::Incheon => "ice",
            Self::Gwangju => "gen",
            Self::Daejeon => "dje",
            Self::Ulsan => "use",
            Self::Sejong => "sje",
            Self::Gyeonggi => "goe",
            Self::Gangwon => "kwe",
            Self::Chungbuk => "cbe",
            Self::Chungnam => "cne",
            Self::Jeonbuk => "jbe",
            Self::Jeonnam => "jne",
            Self::Gyeongbuk => "gbe",
            Self::Gyeongnam => "gne",
            Self::Jeju => "jje"
        }
    }
}

impl TryFrom<&str> for Area {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "서울" | "서울시" | "서울특별시" => Ok(Self::Seoul),
            "부산" | "부산시" | "부산광역시" => Ok(Self::Busan),
            "대구" | "대구시" | "대구광역시" => Ok(Self::Daegu),
            "인천" | "인천시" | "인천광역시" => Ok(Self::Incheon),
            "광주" | "광주시" | "광주광역시" => Ok(Self::Gwangju),
            "대전" | "대전시" | "대전광역시" => Ok(Self::Daejeon),
            "울산" | "울산시" | "울산광역시" => Ok(Self::Ulsan),
            "세종" | "세종시" | "세종특별시" => Ok(Self::Sejong),
            "경기" | "경기도" => Ok(Self::Gyeonggi),
            "강원" | "강원도" => Ok(Self::Gangwon),
            "충북" | "충청북도" => Ok(Self::Chungbuk),
            "충남" | "충청남도" => Ok(Self::Chungnam),
            "전북" | "전라북도" => Ok(Self::Jeonbuk),
            "전남" | "전라남도" => Ok(Self::Jeonnam),
            "경북" | "경상북도" => Ok(Self::Gyeongbuk),
            "경남" | "경상남도" => Ok(Self::Gyeongnam),
            "제주" | "제주도" | "제주특별자치도" => Ok(Self::Jeju),
            _ => Err("해당 지역이 존재하지 않음.")
        }
    }
}