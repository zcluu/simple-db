use std::fs::File;
use std::io::Read;

mod parser;
mod database;

use crate::parser::create::CreateQuery;
use crate::database::table;

fn process_command(query: String, db: &mut database::db::Database) {
    let query = CreateQuery::new(&*query).unwrap();
    let tb = table::Table::new(query);
    db.tables.push(tb);
}


fn main() {
    let db_filename = "simple_db.txt";
    let mut db = database::db::Database::new();
    let mut db_file = match File::open(db_filename) {
        Ok(file) => { Some(file) }
        Err(e) => None
    };
    assert!(!db_file.is_none());
    let mut query: String = String::new();
    let _ = db_file.unwrap().read_to_string(&mut query);
    // println!("{query}");
    process_command(query, &mut db);
    // db.save_disk("data.bin");
    let res = db.load_from_disk("data.bin");
    println!("{:?}", res.unwrap());
    return;
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
