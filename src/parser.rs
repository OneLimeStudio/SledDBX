use std::{net::ToSocketAddrs, ops::Index, vec};
use sled::Db;
use lazy_static::lazy_static;
use std::sync::Mutex;
#[derive(Debug)]
enum Query {
    CreateTable {
        name: String,
        columns: Vec<(String, DataType)>,
    },
    Insert {
        table: String,
        values: Vec<Value>,
    },
    Select {
        table: String,
        columns: Vec<String>,
        condition: Option<Condition>,
    },
    Delete {
        table: String,
        condition: Option<Condition>,
    },
}
// #[derive(Debug)]
// enum data_string {  
    
// }

#[derive(Debug,Clone)]
enum DataType {
    Int,
    Float,
    String,
}
#[derive(Debug)]
enum Condition {
    GreaterThan(String, Value),
    Equals(String, Value),
}

#[derive(Debug)]
struct  Table{
    attribute_name : Vec<String>,
    attribute_type :Vec<DataType>,
}
#[derive(Debug,Clone)]
enum Value {
    Int(i32),
    String(String),
    Float(f32),
}


lazy_static! {
    static ref GLOBAL_TABLE: Mutex<Option<Table>> = Mutex::new(None);
}
fn initialize_global_attribute(name: Vec<String>, data_type: Vec<DataType>) {
    let mut table = GLOBAL_TABLE.lock().unwrap();
    *table = Some(Table { attribute_name:name, attribute_type: data_type });
}
pub fn parse_query(query : &str) -> Result<(), String> {
    let tokens : Vec<&str> = query.split_whitespace().collect();

    if tokens.is_empty() {
        return Err(String::from("Query Empty"));
    }
    match tokens[0].to_uppercase().as_str() {
        "CREATE" => {let x = create_parser(&tokens);
                    },
        "INSERT" => {let y = insert(&tokens);},
        "SELECT" => select(),
        "DELETE" => delete(),
        _ => return  Err(String::from("No Such Query")),
    }
    Ok(())
    
}
fn select(){
    
}


fn create_parser(tokens : &Vec<&str>) -> Result<(), String> {
    let (table_name,attribute_name,attribute_type)  = createtable(tokens);
    //println!("{}",table_name);
    for i in attribute_type.iter(){
        match i {
            DataType::String =>println!("STRING"),
            DataType::Float =>println!("Float"),
            DataType::Int =>println!("Int"),

        }
    }
    // initialize_global_table(attribute_type);
    let name: Vec<String> = attribute_name.iter().map(|s| s.to_string()).collect();
    let db = Db::open(table_name);
    match  db {
        Ok(Db)=>println!("Database Created"),
        _=>{},
    }
    initialize_global_attribute(name, attribute_type);
    Ok(())
}

fn createtable<'a>(tokens: &'a [&'a str]) -> (&'a str, Vec<&'a str>, Vec<DataType>) {
    let mut attribute_type: Vec<DataType> = Vec::new();
    let mut attribute_name: Vec<&'a str> = Vec::new();
    
    let table_name = tokens[2]; 
    match tokens[1] {
        "TABLE" => {
            for (index, val) in tokens.iter().enumerate() {
                if index < 4 { continue; } // Skip "CREATE", "TABLE", table name, and "("
                
                match *val {
                    "STRING" | "INT" | "FLOAT" => {
                       
                        attribute_name.push(tokens[index - 1]);
                        
                        attribute_type.push(match *val {
                            "STRING" => DataType::String,
                            "INT" => DataType::Int,
                            "FLOAT" => DataType::Float,
                            _ => unreachable!(), 
                        });
                    },
                    _ => continue, 
                }   
            }
        },
        _ => println!("No Such Query"),
    }

    (table_name, attribute_name, attribute_type)
}
fn delete(){

}
fn insert(tokens: &[&str]) -> Result<(),sled::Error>{
    let mut values: Vec<Value> = Vec::new();
    let mut table = GLOBAL_TABLE.lock().unwrap();
    let db = Db::open(tokens[2])?;
    if let Some(table) = &mut *table {
        for (index, val) in tokens.iter().enumerate() {
            if index < 4 {
                continue;
            }

            if let Some(data_type) = table.attribute_type.get(index - 4) {
                match data_type {
                    DataType::Int => {
                        if let Ok(num) = val.parse::<i32>() {
                            values.push(Value::Int(num));
                        }
                    }
                    DataType::String => {
                        values.push(Value::String(val.to_string()));
                    }
                    DataType::Float => {
                        if let Ok(num) = val.parse::<f32>() {
                            values.push(Value::Float(num));
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Table is not initialized.");
        return  Ok(());
        
    }

    println!("Parsed values: {:?}", values);
    let pkey = values[0].clone();
   
    insert_record(values, pkey ,&db)?;
    Ok(())
}


pub fn insert_record( v: Vec<Value>, PrimaryKey: Value, db: &Db) -> Result<(), sled::Error> {
    let set_key = PrimaryKey;

    for (j, i) in v.iter().enumerate() {
       
        let key = match set_key {
            Value::Float(x) => format!("{:.2}:{}", x, j),  
            Value::String(ref y) => format!("{}:{}", y, j),   
            Value::Int(x) => format!("{}:{}", x, j),    

        };
        let xd = match set_key {
            Value::Float(x) => println!("Primary KEy {}", x),  
            Value::String(ref y) => {println!("Primary KEy {}", y)},   
            Value::Int(x) => println!("Primary KEy {}", x),    

        };

        let value = match i {
            Value::Float(x) => x.to_string().into_bytes(),
            Value::String(y) => y.clone().into_bytes(),
            Value::Int(x) => x.to_string().into_bytes(),
        };

        db.insert(key, value)?;  
    }

    Ok(())
}