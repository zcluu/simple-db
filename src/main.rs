use std::io::{Read, stdin, stdout, Write};

mod parser;
mod database;
mod utils;
mod sys_command;

use crate::parser::{
    create::CreateQuery,
    insert::InsertQuery,
    select::SelectQuery,
};
use crate::database::table::Table;
use crate::utils::{CommandType, DbSystem, SysCommand};

fn process_sys_command(query: String, db: &mut database::db::Database) {
    match SysCommand::new(query.clone()) {
        SysCommand::CreateDatabase => { sys_command::create_db(query).unwrap() }
        SysCommand::UseDatabase => { sys_command::use_db(query, db).unwrap() }
        SysCommand::DropDatabase => { sys_command::drop_db(query).unwrap() }
        SysCommand::ShowDatabases => { sys_command::show_databases().unwrap() }
        SysCommand::ChangePassword => {}
        SysCommand::SysInfo => {}
    }
}

fn process_command(query: String, db: &mut database::db::Database) {
    match CommandType::new(query.clone()) {
        CommandType::CreateTable => {
            let query = CreateQuery::new(&*query).unwrap();
            let tb = Table::new(query);
            db.create_table(tb);
            db.save_disk().unwrap();
        }
        CommandType::Insert => {
            let query = InsertQuery::new(&*query).unwrap();
            db.insert_row(query.tb_name, query.cols, query.rows);
            db.save_disk().unwrap();
        }
        CommandType::Select => {
            let query = SelectQuery::new(&*query).unwrap();
            let tb_name = query.from.clone();
            let tb = db.get_table(tb_name);
            tb.select_data(query);
        }
        CommandType::ShowTable => {
            let vars = query.split(" ").collect::<Vec<&str>>();
            assert_eq!(vars.len(), 2);
            let tb_name = vars[1].to_string();
            let tb = db.get_table(tb_name);
            tb.print_table_data();
        }
        CommandType::TableInfo => {
            let vars = query.split(" ").collect::<Vec<&str>>();
            assert_eq!(vars.len(), 2);
            let tb_name = vars[1].to_string();
            let tb = db.get_table(tb_name);
            tb.show_info();
        }
        CommandType::System => {
            process_sys_command(query, db);
        }
    }
}


fn main() {
    let mut sys: DbSystem = DbSystem::new();
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
    command = "".to_string();
    let mut db = database::db::Database::new();
    loop {
        if db.db_name.is_empty() {
            print!("simple-db> ");
        } else {
            print!("simple-db[{}]> ", db.db_name);
        }
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
