use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::vec::IntoIter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DataType {
    Float,
    Int,
    Bool,
    String,
    Invalid,
}

impl DataType {
    pub fn new(data_type: String) -> DataType {
        match data_type.to_lowercase().as_str() {
            "float" => DataType::Float,
            "int" => DataType::Int,
            "bool" => DataType::Bool,
            "string" => DataType::String,
            _ => DataType::Invalid,
        }
    }
    pub fn data_type(&self) -> String {
        match self {
            DataType::Float => { "float".to_string() }
            DataType::Int => { "int".to_string() }
            DataType::Bool => { "bool".to_string() }
            DataType::String => { "string".to_string() }
            DataType::Invalid => { "null".to_string() }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ColumnAttr {
    pub name: String,
    pub datatype: DataType,
    pub is_pk: bool,
    pub is_nullable: bool,
    pub default: Option<String>,
}

impl ColumnAttr {
    pub fn new(name: String, data_type: String, is_pk: bool, is_nullable: bool, default: Option<String>) -> ColumnAttr {
        let datatype = DataType::new(data_type);
        ColumnAttr {
            name,
            datatype,
            is_pk,
            is_nullable,
            default,
        }
    }

    pub fn attr(&self) -> HashMap<String, String> {
        let mut row: HashMap<String, String> = HashMap::new();
        row.insert("name".to_string(), self.name.to_string());
        row.insert("datatype".to_string(), self.datatype.data_type());
        row.insert("is_pk".to_string(), self.is_pk.to_string());
        row.insert("is_nullable".to_string(), self.is_nullable.to_string());
        row.insert("default".to_string(), self.default.clone().unwrap_or("None".to_string()));
        row
    }
}

impl std::fmt::Display for ColumnAttr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&*self.name)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ColumnData {
    Int(Vec<i32>),
    Float(Vec<f32>),
    Str(Vec<String>),
    Bool(Vec<bool>),
    None,
}

impl ColumnData {
    pub fn get_all_data(&self) -> Vec<String> {
        match &self {
            ColumnData::Int(x) => x.iter().map(|v| v.to_string()).collect(),
            ColumnData::Float(x) => x.iter().map(|v| v.to_string()).collect(),
            ColumnData::Str(x) => x.iter().map(|v| v.to_string()).collect(),
            ColumnData::Bool(x) => x.iter().map(|v| v.to_string()).collect(),
            ColumnData::None => panic!("Invalid column datatype."),
        }
    }

    pub fn get_data_by_ix(&self, ix: &Vec<usize>) -> Vec<String> {
        let mut data_list: Vec<String> = vec![];
        let all_data = self.get_all_data();
        for i in ix {
            data_list.push(String::from(&all_data[*i]));
        }
        data_list
    }

    pub fn count(&self) -> usize {
        self.get_all_data().len()
    }
}
