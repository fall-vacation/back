use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;

mod db_config;
use db_config::PostgresConfig;

pub struct ConfigLoader {
    loaded: bool,
    config: Option<Config>,
}

#[derive(Deserialize)]
struct Config {
    postgres_config: PostgresConfig,
}

impl ConfigLoader {
    // private ===================================================================================================
    fn get_toml_string(profile:&str) -> String {
        let file_path = format!("src/resources/{profile}.toml");
        let mut toml_file = match File::open(&file_path) {
            Ok(file) => file,
            Err(ref error) if error.kind() == std::io::ErrorKind::NotFound => {
                panic!("{} file not found", file_path)
            },
            Err(error) => {
                panic!("{} file open error {:?}", file_path, error)
            },
        };

        let mut contents = String::from("");
        toml_file.read_to_string(&mut contents).expect("toml file read error");
        contents
    }

    // public ===================================================================================================
    pub fn new() -> ConfigLoader {
        ConfigLoader{
            loaded: false,
            config: None,
        }
    }

    pub fn load_config(&mut self, profile: &str) {
        let toml_string = ConfigLoader::get_toml_string(profile);
        let config: Config = match toml::from_str(toml_string.as_str()) {
            Ok(str) => str,
            Err(error) => {
                panic!("load_config toml::from_str error : {:?}", error)
            }
        };
        self.config = Option::from(config);
        self.loaded = true;
    }

    pub fn get_db_connection_config(self) -> Result<String, ()> {
        if !self.loaded {
            return Err(())
        }

        let postgres_config= self.config.unwrap().postgres_config;
        Ok(postgres_config.get_db_conn_str())
    }
}