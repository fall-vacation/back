use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub user: String,
    pub port: Option<u16>,
    pub password: String,
    pub dbname: String,
}

impl PostgresConfig {
    pub fn get_db_conn_str(&self) -> String {
        format!("host={} user={} password={} dbname={} port={}",
                self.host,
                self.user,
                self.password,
                self.dbname,
                self.port.unwrap(),
        )
    }
}