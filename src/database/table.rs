use crate::parser::create::CreateQuery;
use crate::database;
use std::collections::HashMap;
use crate::database::base::{ColumnAttr, ColumnData, DataType};

pub struct Table {
    pub name: String,
    pub columns: Vec<database::base::ColumnAttr>,
    pub col_map: HashMap<String, database::base::ColumnData>,
}

impl Table {
    pub fn new(cq: CreateQuery) -> Table {
        let tb_name = cq.tb_name;
        let columns = cq.cols;
        println!("{:?}", columns);
        let mut tb_cols: Vec<ColumnAttr> = vec![];
        let mut tb_col_map: HashMap<String, ColumnData> = HashMap::new();
        for column in &columns {
            let col_header = ColumnAttr::new(
                column.name.to_string(), column.datatype.to_string(), column.is_pk,
                column.is_nullable, column.default.clone(),
            );
            tb_cols.push(col_header);
            tb_col_map.insert(column.name.to_string(), match DataType::new(column.datatype.to_string()) {
                DataType::Float => { ColumnData::Float(vec![]) }
                DataType::Int => { ColumnData::Int(vec![]) }
                DataType::Bool => { ColumnData::Bool(vec![]) }
                DataType::String => { ColumnData::Str(vec![]) }
                DataType::Invalid => { ColumnData::None }
            });
        }
        Table {
            name: tb_name,
            columns: tb_cols,
            col_map: tb_col_map,
        }
    }
}
