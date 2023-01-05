use rocket::form::validate::Len;
use sqlx::types::chrono::{NaiveDateTime, NaiveTime, Utc};

pub trait ToQuery {
    fn to_query_string(&self) -> String;
}

impl ToQuery for Option<i32> {
    fn to_query_string(&self) -> String {
        match self {
            Some(data) => { format!("{}", data) },
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

impl ToQuery for String {
    fn to_query_string(&self) -> String {
        format!("'{}'", self)
    }
}

impl ToQuery for Option<NaiveTime> {
    fn to_query_string(&self) -> String {
        match self {
            Some(naive) => naive.format("'%H:%M:%S'").to_string(),
            None => "NULL".to_string(),
        }
    }
}

impl ToQuery for Option<bool> {
    fn to_query_string(&self) -> String {
        match self {
            Some(data) => { format!("{}", data) },
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

pub fn string_to_naive_time(opt_data:&Option<String>) -> Option<NaiveTime> {
    match opt_data {
        Some(data) => {
            match NaiveTime::parse_from_str(data, "%H:%M:%S") {
                Ok(naive) => { Some(naive) },
                Err(e) => {
                    println!("[ERROR] string_to_naive_time : {}", e.to_string());
                    None
                }
            }
        },
        None => { None },
    }
}

// For NaiveTime or NaiveDateTime ============================================================
// ===========================================================================================
pub fn naive_time_to_string(opt_naive:&Option<NaiveTime>) -> Option<String> {
    match opt_naive {
        Some(naive) => {
            Some(naive.format("%H:%M:%S").to_string())
        },
        None => { None },
    }
}

pub fn string_to_naive_date_time(opt_string: &Option<String>) -> Option<NaiveDateTime> {
    match opt_string {
        Some(naive_string) => {
            match NaiveDateTime::parse_from_str(naive_string, "%Y-%m-%d %H:%M:%S"){
                Ok(naive_date_time) => { Some(naive_date_time) },
                Err(e) => {
                    println!("[ERROR] string_to_naive_date_time : {}", e.to_string());
                    None
                }
            }
        },
        None => { None }
    }
}

pub fn string_to_naive_date_time_default_now(opt_string: &Option<String>) -> NaiveDateTime {
    match string_to_naive_date_time(opt_string){
        Some(naive_date_time) => { naive_date_time },
        None => {
            // default 값 : now
            Utc::now().naive_utc()
        }
    }
}

pub fn naive_date_time_to_string(opt_naive: &Option<NaiveDateTime>) -> Option<String> {
    match opt_naive {
        Some(naive) => {
            Some(naive.format("%Y-%m-%d %H:%M:%S").to_string())
        },
        None => { None },
    }
}