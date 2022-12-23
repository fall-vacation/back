use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub enum FarmUrlDivision {
    Homepage,
    NaverBlog,
    Line,
    Instagram,
    Facebook,
    KakaoStory,
    KakaoPage,
    National,
    ETC,
}

impl FarmUrlDivision {
    pub fn to_string(&self) -> String {
        match self {
            FarmUrlDivision::Homepage => "Homepage".to_string(),
            FarmUrlDivision::NaverBlog => "NaverBlog".to_string(),
            FarmUrlDivision::Line => "Line".to_string(),
            FarmUrlDivision::Instagram => "Instagram".to_string(),
            FarmUrlDivision::Facebook => "Facebook".to_string(),
            FarmUrlDivision::KakaoStory => "KakaoStory".to_string(),
            FarmUrlDivision::KakaoPage => "KakaoPage".to_string(),
            FarmUrlDivision::National => "National".to_string(),
            FarmUrlDivision::ETC => "ETC".to_string(),
        }
    }

    pub fn get_enum(str: &String) -> FarmUrlDivision {
        return match str.as_str() {
            "Homepage" => FarmUrlDivision::Homepage,
            "NaverBlog" => FarmUrlDivision::NaverBlog,
            "Line" => FarmUrlDivision::Line,
            "Instagram" => FarmUrlDivision::Instagram,
            "Facebook" => FarmUrlDivision::Facebook,
            "KakaoStory" => FarmUrlDivision::KakaoStory,
            "KakaoPage" => FarmUrlDivision::KakaoPage,
            "National" => FarmUrlDivision::National,
            _ => FarmUrlDivision::ETC,
        }
    }
}