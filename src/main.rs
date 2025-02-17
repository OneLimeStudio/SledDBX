use std::fmt::format;

use sled::Db;
mod parser;
#[derive (Debug)]
enum _data_type {
    _Number(i32),
    _Float(f64),
    _String(String)
}
fn main() -> Result<(), sled::Error> {
    parser::parse_query(&String::from("CREATE TABLE XD ( HELLO STRING LOL STRING )"));
    parser::parse_query(&String::from("INSERT INTO XD ( 1 ok )"));
    parser::parse_query(&String::from("INSERT INTO XD ( 2 ok )"));
    parser::parse_query(&String::from("INSERT INTO XD ( 3 ok )"));
    parser::parse_query(&String::from("SELECT * FROM XD"));

    
    Ok(())
}


