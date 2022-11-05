// extern crate fall_vacation_back;
//
// use fall_vacation_back::config;
//
// #[test]
// fn db_conn_test() {
//     let mut configLoader = config::ConfigLoader::new();
//     configLoader.load_config("local");
//     match configLoader.get_db_connection_config() {
//         Ok(str) => {
//             println!("{}", str)
//         },
//         Err(()) => {}
//     }
// }