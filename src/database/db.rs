use crate::database::table::Table;
use serde_json;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

    pub fn save_disk(&self, filename: &str) -> io::Result<()> {
        let serialized_data = serde_json::to_string(&self)?;
        let mut file = File::create(filename)?;
        file.write_all(serialized_data.as_bytes())?;
        Ok(())
    }
    pub fn load_from_disk(&self, filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let database = serde_json::from_reader(file)?;
        Ok(database)
    }
}