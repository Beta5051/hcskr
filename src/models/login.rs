pub enum LoginType {
    School,
    University,
    Office
}

impl LoginType {
    pub fn get_code(&self) -> &'static str {
        match self {
            LoginType::School => "school",
            LoginType::University => "univ",
            LoginType::Office => "office"
        }
    }
}