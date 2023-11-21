mod parser;
mod database;

fn main() {
    println!("Hello, world!");
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
