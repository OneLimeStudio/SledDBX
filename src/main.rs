
mod parser;
use std::io;




#[derive (Debug)]
enum _data_type {
    _Number(i32),
    _Float(f64),
    _String(String)
}
fn main() -> Result<(), sled::Error> {
    // parser::parse_query(&String::from("CREATE TABLE XD ( HELLO STRING LOL STRING )"));
    // parser::parse_query(&String::from("INSERT INTO XD ( 1 ok )"));
    // parser::parse_query(&String::from("INSERT INTO XD ( 2 ok )"));
    // parser::parse_query(&String::from("INSERT INTO XD ( 3 ok )"));
    // parser::parse_query(&String::from("SELECT * FROM XD"));
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to Read Query");
        let guess = guess.trim();
        if guess == "QUIT"{
            break;
        }
        let _x = parser::parse_query(guess);

        
    }

    Ok(())
}


