use crate::parser::create::CreateQuery;
use std::collections::HashMap;
use prettytable::row;
use serde::{Serialize, Deserialize};
use crate::database::base::{ColumnAttr, ColumnData, DataType};
use prettytable::{Cell, Row, Table as PTable};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<ColumnAttr>,
    pub col_map: HashMap<String, ColumnData>,
}

impl Table {
    pub fn new(cq: CreateQuery) -> Table {
        let tb_name = cq.tb_name;
        let columns = cq.cols;
        // println!("{:?}", columns);
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

    pub fn insert_row(&mut self, cols: Vec<String>, rows: Vec<Vec<String>>) {
        for col_ix in 0..cols.len() {
            let col_name = &cols[col_ix];
            if let Some(col_data) = self.col_map.get_mut(&col_name.to_string()) {
                for row in &rows {
                    let col_val = &row[col_ix];
                    match col_data {
                        ColumnData::Int(v) => { v.push(col_val.parse::<i32>().unwrap()) }
                        ColumnData::Float(v) => { v.push(col_val.parse::<f32>().unwrap()) }
                        ColumnData::Str(v) => { v.push(col_val.to_string()) }
                        ColumnData::Bool(v) => { v.push(col_val.parse::<bool>().unwrap()) }
                        ColumnData::None => { panic!("Invalid column datatype.") }
                    }
                }
            }
        }
    }


    pub fn print_table_data(&self) {
        let mut p_table = PTable::new();

        let cnames = self
            .columns
            .iter()
            .map(|col| col.name.to_string())
            .collect::<Vec<String>>();

        let header_row = Row::new(
            cnames
                .iter()
                .map(|col| Cell::new(&col))
                .collect::<Vec<Cell>>(),
        );

        let first_col_data = self.col_map.get(&self.columns.first().unwrap().name).unwrap();
        let num_rows = first_col_data.count();
        let mut print_table_rows: Vec<Row> = vec![Row::new(vec![]); num_rows];

        for col_name in &cnames {
            let col_val = self
                .col_map
                .get(col_name)
                .expect("Can't find any rows with the given column");
            let columns: Vec<String> = col_val.get_all_data();

            for i in 0..num_rows {
                print_table_rows[i].add_cell(Cell::new(&columns[i]));
            }
        }

        p_table.add_row(header_row);
        for row in print_table_rows {
            p_table.add_row(row);
        }

        p_table.printstd();
    }
}
