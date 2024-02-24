use rusqlite::Connection;
use std::env;
use std::path::Path;
use std::collections::{HashMap, BTreeMap};
use rusqlite::types::Value;

struct DataBase{
conn: Connection
}

enum Commands{
Tables,
Info(String),
Records(String),
}

impl DataBase {
	fn new(conn: Connection) -> DataBase {
		DataBase{
			conn
		}
	}
	
	fn get_tables(&self) -> Vec<String> {
		let mut stmt = self
				.conn
				.prepare("SELECT name FROM sqlite_master WHERE type='table'")
				.expect("Failed to prepare query");
		let tables = stmt
				.query_map([], |row| { row.get(0)})
				.expect("Error while getting tables")
				.map(|result| result.unwrap())
				.collect();
		tables
	}
	
	fn records(&self, name:&String) -> Option<Vec<HashMap<String, Value>>> {
		let mut stmt = self
				.conn
				.prepare(format!("SELECT * FROM {}", name).as_str())
				.expect(format!("Table {} doesn't exist", name).as_str());
		
		let columns_name: Vec<String> = stmt.column_names().into_iter().map(|name| name.to_string()).collect();		
		let mut records = Vec::new();
		let query_result = stmt.
					query_map([], |row| {
							let mut columns = HashMap::new();
							for (index, name) in columns_name.iter().enumerate() {
									if let Some(value) = row.get(index).ok(){
										columns.insert(name.clone(), value);
									}
								}
							Ok(columns)
						}).ok()?;
		for row in query_result {
			records.push(row.ok()?);
		}
		Some(records)
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	
	if !validate_path(args[1].as_str()) {
	println!("path {} doesn't exist please enter a valid path", args[1]);
	return;
	}
	if args.len() < 3 {
	println!("Number of arguments is invalid");
	return;
	}

	if let Some(commands) = check_args(&args){

		if let Some(db) = connect(&args[1]) {
			validate_commands(&db, commands);
			}
		else {
			println!("faild connecting to the database");
		}
	}else{
		println!("Command {} is not valid", args[2]);
	}
}

fn check_args(args: &Vec<String>) -> Option<Commands>{
	let commands = match args[2].as_str() {
		"-tables" => Commands::Tables,
		"-records" => {
				if args.len() <4 {
						println!("Usage rusql dbpath command tablename");
						return None;
					}
				Commands::Records(args[3].clone())
				},
		"-info" => return None,
		_ => return None,

	};
	Some(commands)
}

fn validate_commands(db:&DataBase, command: Commands) {
	match command {
	Commands::Tables => get_tables_name(db),
	Commands::Records(table_name) => records(db, table_name),
	Commands::Info(_table_name) => println!("Command not implemented yet"),
	};
}

fn validate_path(path:&str) -> bool {
	if Path::new(path).exists(){return true;}
	else{return false;} 							
}

fn connect(path: &String) -> Option<DataBase>{
	let conn = Connection::open(path);
	match conn {
			Ok(conn) => {
					return Some(DataBase::new(conn));
				}
			Err(err) => {
					println!("Error occured {}", err);
					return None;
				}
		}

} 

fn get_tables_name(db: &DataBase) {
	let tables = db.get_tables();
	println!("Tables: ");
	for (index, table) in tables.iter().enumerate() {
		println!("{}: {}", index + 1, table);
	}
	println!("");
}

fn records(db: &DataBase, table: String) {
    if let Some(records) = db.records(&table) {
        if !records.is_empty() {
            println!("Records:");

            let first_record = records.first().unwrap();
            let mut header = String::new();
            let mut keys_sorted: Vec<_> = first_record.keys().collect();
            keys_sorted.sort();
            for key in keys_sorted {
                header += &format!("{:<12} ", key);
            }
            println!("{}", header);
            
            for record in records.iter() {
                let mut row = String::new();
                let record_ordered: BTreeMap<_, _> = record.iter().collect();
                for (_, value) in &record_ordered {
                    match value {
                        Value::Text(text) => row += &format!("{:<12} ", text),
                        Value::Integer(int) => row += &format!("{:<12} ", int),
                        Value::Real(real) => row += &format!("{:<12} ", real),
                        Value::Blob(blob) => row += &format!("{:<12?} ", blob),
                        Value::Null => row += "NULL       ",
                    }
                }
                println!("{}", row);
            }
        } else {
            println!("There are no records to fetch");
        }
    } else {
        println!("Table '{}' does not exist", table);
    }
}
