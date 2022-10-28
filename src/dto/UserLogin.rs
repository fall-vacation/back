use chrono::{DateTime, Utc};

struct UserLogin{
    email_address : String,
    access_token: String,
    access_expired: DateTime<Utc>,
    refresh_token: String,
    refresh_expired: DateTime<Utc>,
}