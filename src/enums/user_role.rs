use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum UserRole {
    ADMIN,
    FARMER,
    USER,
    NONE,
}

impl UserRole {
    pub fn get_string(&self) -> String {
        return match self {
            UserRole::ADMIN => { "ADMIN".to_string() },
            UserRole::FARMER => { "FARMER".to_string() },
            UserRole::USER => { "USER".to_string() },
            UserRole::NONE => { panic!("no match role") },
        }
    }

    pub fn get_enums(role:&str) -> UserRole {
        match role {
            "ADMIN" => { UserRole::ADMIN },
            "FARMER" => { UserRole::FARMER },
            "USER" => { UserRole::USER },
            "NONE" | _ => { UserRole::NONE },
        }
    }

    pub fn validation(&self) -> bool {
        match self {
            UserRole::NONE => false,
            _ => true,
        }
    }
}