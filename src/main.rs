use std::fs::File;
use std::io::{Read, stdin, stdout, Write};

mod parser;
mod database;
mod utils;

use crate::parser::{
    create::CreateQuery,
    insert::InsertQuery,
    select::SelectQuery,
};
use crate::database::table::Table;
use crate::utils::{CommandType, DbSystem, Password};

fn process_command(query: String, db: &mut database::db::Database) {
    match CommandType::new(query.clone()) {
        CommandType::CreateTable => {
            let query = CreateQuery::new(&*query).unwrap();
            let tb = Table::new(query);
            db.create_table(tb);
        }
        CommandType::Insert => {
            let query = InsertQuery::new(&*query).unwrap();
            db.insert_row(query.tb_name, query.cols, query.rows);
        }
        CommandType::Select => {
            let query = SelectQuery::new(&*query).unwrap();
        }
        CommandType::System => {
            println!("System command.");
        }
    }
}


fn main() {
    let mut sys: DbSystem = DbSystem::new();
    // let mut db = database::db::Database::new();
    // db.load_from_disk("data.bin").unwrap();
    // let db_filename = "simple_db.txt";
    // let mut db_file = match File::open(db_filename) {
    //     Ok(file) => { Some(file) }
    //     Err(e) => None
    // };
    // assert!(!db_file.is_none());
    // let mut query: String = String::new();
    // let _ = db_file.unwrap().read_to_string(&mut query);
    // println!("{query}");
    // let create_sql = "CREATE TABLE example_table (
    //     id INT PRIMARY KEY,
    //     name VARCHAR(100) NOT NULL DEFAULT Tom,
    //     age INT NOT NULL,
    // );";
    // // process_command(create_sql.to_string(), &mut db);
    //
    // let insert_sql = "INSERT INTO example_table (id, name, age) VALUES (1, 'John Doe', '25'),(2, 'Tom', '30');";
    // process_command(insert_sql.to_string(), &mut db);
    let mut command = String::new();
    loop {
        print!("login: ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut command)
            .expect("Error while trying to read from stdin");
        let vars = command.trim().split(" ").collect::<Vec<&str>>();
        let username = vars[0].to_string();
        let password = vars[1].to_string();
        let is_login = sys.login(username.clone(), password.clone());
        if !is_login {
            println!("Invalid username or password.")
        } else {
            println!("Welcome back {}.", username);
            break;
        }
    }

    let mut db = database::db::Database::new();
    loop {
        print!("simple-db> ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut command)
            .expect("Error while trying to read from stdin");
        process_command(command.trim().to_string(), &mut db);
        command = "".to_string();
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::create::CreateQuery;
    use crate::database::table;

    #[test]
    fn test_create_table_new() {
        let sql = "CREATE TABLE employees (
        id INT PRIMARY KEY,
        name VARCHAR(100) NOT NULL DEFAULT Tom,
        role VARCHAR(100),
        department_id INT DEFAULT 0,
        abcd_id INT DEFAULT 0,
        abcd_x INT DEFAULT 0,
        email VARCHAR(100) UNIQUE,
        FOREIGN KEY (department_id) REFERENCES departments(id),
        FOREIGN KEY (abcd_id) REFERENCES abcds(id),
        FOREIGN KEY (abcd_x) REFERENCES abcds(x)
    );";
        let query = CreateQuery::new(sql).unwrap();
        table::Table::new(query);
    }
}
