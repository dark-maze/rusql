use rusqlite::Connection;
use std::env;
use std::path::Path;
use std::collections::{HashMap, BTreeMap};
use rusqlite::types::Value;
use rand::Rng;

struct DataBase{
conn: Connection
}

enum Commands{
Tables,
Info(String),
Records(String, Option<Vec<String>>),
RecordsNo(String),
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
		let mut stmt = match self
				.conn
				.prepare(format!("SELECT * FROM {}", name).as_str())
				{
					Ok(stmt) => stmt,
					Err(_err) => {
						return None;
					},
				};
		
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

	fn get_no_of_records(&self, table: &String) -> Option<usize> {
		let mut stmt = match self
				.conn
				.prepare(format!("SELECT COUNT(*) FROM {}", table).as_str()){
					Ok(stmt) => stmt,
					Err(err) => {
						println!("Table doesn't exist");
						return None;
					}
				};
		let count = stmt
				.query_row([], |row| row.get(0)).ok()?;
		Some(count)
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
		println!("invalid arguments");
	}
}

fn check_args(args: &Vec<String>) -> Option<Commands>{
	let commands = match args[2].as_str() {
		"-tables" => Commands::Tables,
		"-records" => {
				if args.len() <4 {
						return None;
					}
				if args.len() > 4 {Commands::Records(args[3].clone(), Some(args[4..].to_vec()))}
				else {Commands::Records(args[3].clone(), None)}
				},
		"-records--no" => {
					if args.len() <4 {
						return None;
						}
					Commands::RecordsNo(args[3].clone())},
		"-info" => return None,
		_ => return None,

	};
	Some(commands)
}

fn validate_commands(db:&DataBase, command: Commands) {
	match command {
	Commands::Tables => get_tables_name(db),
	Commands::Records(table_name, records_size) => records(db, table_name, records_size),
	Commands::RecordsNo(table_name) => get_number_of_records(db, table_name),
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

fn records(db: &DataBase, table: String, records_to_show: Option<Vec<String>>) {

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
		let records_len = db.get_no_of_records(&table).expect("Error getting number of records");
		let no_of_records: usize = match get_records_len(records_to_show, records_len) {
			Some(val) => val,
			None => {
				println!("Please enter a valid number");
				let val:usize = 0;
				val
			},
		};

		for (index, record) in records.iter().enumerate() {
			if index as usize == no_of_records {break;}
			if no_of_records > records_len { 
			println!("\nSorry number provided is larger than the length of the records");
			break;
			}
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

fn get_number_of_records(db: &DataBase, table: String) {
	match db.get_no_of_records(&table) {
			Some(val) => println!("Number of records {}", val),
			None => return,
		};
}

fn get_records_len(arg: Option<Vec<String>>, no_of_records: usize) -> Option<usize> {
	let no_of_records = match arg{	
		Some(arg) => {	match arg[0].as_str() {
				"-r" => {
						let mut rng = rand::thread_rng();
						let num: Option<usize> = if arg[1].chars().all(|c| c.is_digit(10)) {
									    Some(arg[1].parse().unwrap_or_default())
									} else {
									    None
									};
						let num = num.expect("Please enter a vaild number");
						if num> no_of_records {
								println!("Number entered is larger than the number of records");
							}
						let random: usize= rng.gen_range(1..=num);
						Some(random)
					},
				"-n" =>{
						if arg[1].chars().all(|c| c.is_digit(10))
							{
							let num_records: usize = arg[1].parse::<usize>().expect("Faild to convert to numnber");
							Some(num_records)
							}
						else {
							println!("Please enter a valid number");
							Some(0)
							}
						},
				"-a" => Some(no_of_records),
				_ => None,
				}},
		None => {
				println!("{:?}", arg);
				None

			},
	};
		no_of_records
}
