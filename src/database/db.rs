use crate::database::table::Table;

pub struct Database {
    pub tables: Vec<Table>,
}

impl Database {
    pub fn new() -> Database {
        return Database { tables: vec![] };
    }

    pub fn check_table(&self, tb_name: String) -> bool {
        self.tables.iter().any(|v| v.name == tb_name)
    }

    pub fn get_table(&self, tb_name: String) -> &Table {
        for tb in &self.tables {
            if tb.name == tb_name {
                return tb;
            }
        }
        panic!("Table {} is not existed.", tb_name)
    }
}