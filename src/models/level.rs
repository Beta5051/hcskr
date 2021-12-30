use std::convert::TryFrom;

pub enum Level {
    Kinder,
    Elementary,
    Middle,
    High,
    Special
}

impl Level {
    pub fn get_code(&self) -> &'static str {
        match self {
            Self::Kinder => "1",
            Self::Elementary => "2",
            Self::Middle => "3",
            Self::High => "4",
            Self::Special => "5",
        }
    }
}

impl TryFrom<&str> for Level {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "유" | "유치" | "유치원" => Ok(Self::Kinder),
            "초" | "초등" | "초등학교" => Ok(Self::Elementary),
            "중" | "중등" | "중학교" => Ok(Self::Middle),
            "고" | "고등" | "고등학교" => Ok(Self::High),
            "특" | "특수" | "특수학교" => Ok(Self::Special),
            _ => Err("해당 학교 유형이 존재하지 않음.")
        }
    }
}