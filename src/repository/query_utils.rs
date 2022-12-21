use rocket::form::validate::Len;
use sqlx::types::chrono::NaiveTime;

pub trait ToQuery {
    fn to_query_string(&self) -> String;
}

impl ToQuery for Option<i32> {
    fn to_query_string(&self) -> String {
        match self {
            Some(data) => { format!("'{}'", data) },
            None => "NULL".to_string(),
        }
    }
}

impl ToQuery for Option<String> {
    fn to_query_string(&self) -> String {
        match self {
            Some(data) => match self.len() {
                0 => { "NULL".to_string() }
                _ => { format!("'{}'", data) }
            },
            None => "NULL".to_string(),
        }
    }
}

impl ToQuery for Option<NaiveTime> {
    fn to_query_string(&self) -> String {
        match self {
            Some(naive) => naive.format("%H:%M:%S").to_string(),
            None => "NULL".to_string(),
        }
    }
}

impl ToQuery for Option<bool> {
    fn to_query_string(&self) -> String {
        match self {
            Some(data) => { format!("'{}'", data) },
            None => "NULL".to_string(),
        }
    }
}

impl ToQuery for Vec<i32> {
    fn to_query_string(&self) -> String {
        let iter : Vec<String> = self.iter().map( |&x| x.to_string()).collect();
        iter.join(", ")
    }
}


// trait -> struct 로 되는지 확인 필요
// impl <T:Display + 'static> ToQuery for Option<T> {
//     fn to_string_test(&self) -> String {
//         match self {
//             Some(data) => {
//                 if data.type_id() == TypeId::of::<String>(){
//                     let str_data = data.as_any();
//                     // str_data.len()
//                 }
//                 format!("'{}'", data)
//             },
//             None => "NULL".to_string(),
//         }
//     }
// }