use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
struct Config {
    postgres_config: PostgresConfig,
}

#[derive(Deserialize)]
struct PostgresConfig {
    host: String,
    user: String,
    port: Option<u16>,
    password: String,
    dbname: String,
}

fn get_toml_string(profile:&str) -> String {
    let filename = format!("src/resources/{profile}.toml");
    let mut toml_file = match File::open(&filename) {
        Ok(file) => file,
        Err(ref error) if error.kind() == std::io::ErrorKind::NotFound => {
            panic!("{} file not found", filename)
        },
        Err(error) => {
            panic!("{} file open error {:?}", filename, error)
        },
    };

    let mut contents = String::from("");
    toml_file.read_to_string(&mut contents).expect("toml file read error");
    contents
}

pub fn get_db_connection_config(profile:&str) -> String {
    let toml_string = get_toml_string(profile);
    let config: Config = toml::from_str(toml_string.as_str()).unwrap();

    format!("host={} user={} password={} dbname={} port={}",
            config.postgres_config.host,
            config.postgres_config.user,
            config.postgres_config.password,
            config.postgres_config.dbname,
            config.postgres_config.port.unwrap(),
    )
}