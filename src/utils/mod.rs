use std::fmt::Display;
use sqlx::types::chrono::NaiveTime;

pub fn string_to_naive_time(opt_data:&Option<String>) -> Option<NaiveTime> {
    match opt_data {
        Some(data) => {
            match NaiveTime::parse_from_str(data, "%H:%M:%S") {
                Ok(naive) => { Some(naive) },
                Err(e) => {
                    println!("[LOG] string_to_naive_time : {}", e.to_string());
                    None
                }
            }
        },
        None => { None },
    }
}

pub fn naive_time_to_string(opt_naive:&Option<NaiveTime>) -> Option<String> {
    match opt_naive {
        Some(naive) => {
            Some(naive.format("%H:%M:%S").to_string())
        },
        None => { None },
    }
}

pub fn to_query_string<T:Display>(data: &Option<T>) -> String {
    match data {
        Some(data) => {
            format!("'{}'", data)
        },
        None => "NULL".to_string(),
    }
}