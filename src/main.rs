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

    // let v : Vec<_data_type> = vec![_data_type::_Float(10.0),_data_type::_Float(11.0) ];
    // let primarykey = _data_type::_String(String::from("Hello"));
    // let q : Vec<_data_type> = vec![_data_type::_Float(10.0),_data_type::_Float(11.0) ];
    // let primarykey2 = _data_type::_String(String::from("Fuck"));
    // insert_record(v, primarykey, &db)?;
    // insert_record(q, primarykey2, &db)?;
    // if let Some(value) = db.get("Fuck:0")? {
    //     let value_str = String::from_utf8(value.to_vec()).unwrap();
    //     println!("Value: {}", value_str);
    // } else {
    //     println!("Key not found!");
    // }
    
    parser::parse_query(&String::from("CREATE TABLE XD ( HELLO STRING LOL STRING )"));
    parser::parse_query(&String::from("INSERT INTO XD ( 1 ok )"));
    let db = Db::open("XD")?;
    if let Some(value) = db.get("1:1")? {
        let value_str = String::from_utf8(value.to_vec()).unwrap();
        println!("Value: {}", value_str);
    } else {
        println!("Key not found!");
    }
    
    
    db.flush()?;

    Ok(())
}


