use rusqlite::{Connection};
use std::env;
use std::path::Path;
use std::process::Command;
use std::io::stdin;

struct DataBase{
conn: Connection
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
	
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() == 0 {
		eprint!("Error no args been received");
	}
	get_command(args);
}

fn get_command(command: Vec<String>){
	match command[1].as_str() {
		"connect" => {
				if Path::new(&command[2]).exists() {
						if let Some(db) = connect(&command[2]){
								run(db);
							}else{ println!("Error occured while trying to connect");} 
								
					}
				else {	
				println!("Error occured while connecting to {}, please check if the file exist", &command[2]);				
				}
			}
		_ => {
			println!("command {} is invalid", &command[1]);
			}
	}

}

fn run(db:DataBase){
	clear_terminal();
	loop{
		println!("Please Enter Your Command\n");
		let mut input = String::new();	
		stdin()
			.read_line(&mut input)
			.expect("faild to get the command");
		let input = input.trim();
		println!("");
		match input{
			"tables" => fetch_table(&db),
			"clear" => clear_terminal(),
			"exit" => break,
			_ => println!("Command {} Not implemented yet", input),
		}
	}
}


fn clear_terminal(){
	let _ = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
    	} else {
        Command::new("clear")
            .status()

	};
}

fn connect(path: &String) -> Option<DataBase>{
	let conn = Connection::open(path);
	match conn {
			Ok(conn) => {
					println!("Database {} opened", path);
					return Some(DataBase::new(conn));
				}
			Err(err) => {
					println!("Error occured {}", err);
					return None;
				}

		}

}

fn fetch_table(db: &DataBase) {
	let tables = db.get_tables();
	println!("Tables Name");
	for (index, table) in tables.iter().enumerate() {
		println!("{}: {}", index + 1, table);
	}
	println!("");
}
