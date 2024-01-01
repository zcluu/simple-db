use crate::database::db::Database;
use crate::database::table::PrettyTable;
use std::collections::HashMap;
use std::fs;

fn database_exists(db_name: &str) -> bool {
    let file_path = format!("sql_files/{}.bin", db_name);
    fs::metadata(file_path).is_ok()
}

pub fn create_db(command: String) -> std::io::Result<()> {
    let vars = command.split(" ").collect::<Vec<&str>>();
    assert_eq!(vars.len(), 3);
    let db_name = vars[2];
    let mut db = Database::new();
    db.set_dbname(db_name.to_string());
    db.save_disk().unwrap();
    Ok(())
}

pub fn use_db(command: String, db: &mut Database) -> std::io::Result<()> {
    let vars = command.split(" ").collect::<Vec<&str>>();
    assert_eq!(vars.len(), 3);
    let db_name = vars[2];
    let file_path = format!("sql_files/{}.bin", db_name);
    if !database_exists(db_name) {
        panic!("===> {db_name} doesn't existed.")
    }
    db.load_from_disk(file_path.as_str()).unwrap();
    println!("You are using {}.", db.db_name);
    Ok(())
}

pub fn drop_db(command: String) -> std::io::Result<()> {
    let vars = command.split(" ").collect::<Vec<&str>>();
    assert_eq!(vars.len(), 3);
    let db_name = vars[2];
    let file_path = format!("sql_files/{}.bin", db_name);
    fs::remove_file(file_path)?;
    println!("You have dropped {}", db_name);
    Ok(())
}

pub fn show_databases() -> std::io::Result<()> {
    let files = fs::read_dir("sql_files")?;

    let mut databases = Vec::new();

    for file in files {
        if let Ok(entry) = file {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".bin") {
                    let db_name = file_name.trim_end_matches(".bin").to_string();
                    databases.push(db_name);
                }
            }
        }
    }
    let db_header = vec!["Database".to_string()];
    let mut col_values: HashMap<String, Vec<String>> = HashMap::new();
    col_values.insert("Database".to_string(), databases);
    let pt = PrettyTable::new("".to_string(), db_header, col_values);
    println!("{}", pt);
    Ok(())
}
