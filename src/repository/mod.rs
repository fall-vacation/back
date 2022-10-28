extern crate postgres;

use postgres::{Client, NoTls, Row};

pub struct DBManager {
    connection: bool,
    client: Client,
}

impl DBManager {
    pub fn new(args: &str) -> DBManager {
        match Client::connect(args, NoTls) {
            Ok(client) => {
                DBManager {
                    connection: true,
                    client
                }
            },
            Err(error) => {
                panic!("db connection error : {:?}", error)
            }
        }
    }

    pub fn query(&mut self, query: &str) -> Vec<Row> {
        self.client.query(query, &[]).unwrap()
    }
}

