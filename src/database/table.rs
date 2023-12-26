use crate::parser::create::CreateQuery;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Formatter};
use serde::{Serialize, Deserialize};
use crate::database::base::{ColumnAttr, ColumnData, DataType};
use prettytable::{Cell, Row, row, Table as PTable};
use crate::parser::select::{BinaryOpCus, Condition, SelectQuery};

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

    pub fn create(name: String, columns: Vec<ColumnAttr>, col_map: HashMap<String, ColumnData>) -> Table {
        Table {
            name,
            columns,
            col_map,
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

    pub fn select_data(&self, query: SelectQuery) {
        let condition = &query.condition;
        let mut projection = &query.projection;
        let mut proj_set: HashSet<String> = HashSet::new();
        let mut proj_loc: HashMap<String, usize> = HashMap::new();
        let mut proj_loc_ix = 0;
        for proj in projection {
            if proj.eq("*") {
                let all_cols = self.columns
                    .iter()
                    .map(|c| c.name.clone())
                    .collect::<Vec<String>>();
                for col in &all_cols {
                    proj_loc.insert(col.to_string(), proj_loc_ix);
                    proj_loc_ix += 1;
                }
                proj_set.extend(all_cols);
            } else {
                proj_set.insert(proj.to_string());
                if proj_loc.get(proj).is_none() {
                    proj_loc.insert(proj.to_string(), proj_loc_ix);
                    proj_loc_ix += 1;
                }
            }
        }
        let mut binding = proj_set.iter().map(|c| c.to_string()).collect::<Vec<String>>();
        binding.sort_by_key(|k| proj_loc.get(k));
        projection = &binding;

        let mut rows: Vec<HashMap<String, String>> = vec![];
        let first_col = self.columns.first().unwrap().clone();
        let first_col_name = &first_col.name;
        let row_nums = self.col_map.get(first_col_name).unwrap().count();
        for rid in 0..row_nums {
            let mut row_col: HashMap<String, String> = HashMap::new();
            for (key, val) in &self.col_map {
                row_col.insert(key.parse().unwrap(), val.get_data_by_ix(&vec![rid]).first().unwrap().parse().unwrap());
            }
            rows.push(row_col);
        }
        match condition {
            None => {}
            Some(con) => {
                rows = rows.iter()
                    .filter(|&row| self.evaluate_condition(row, con))
                    .clone()
                    .map(
                        |row| row.iter().filter(|(col, _)| projection.contains(&col))
                            .map(|(k, v)| (k.to_owned(), v.to_owned()))
                            .collect::<HashMap<String, String>>()
                    )
                    .collect::<Vec<HashMap<String, String>>>();
            }
        }

        let pt = PrettyTable::create("".to_string(), projection.to_vec(), rows);
        println!("{pt}");
    }

    pub fn evaluate_condition(&self, row: &HashMap<String, String>, condition: &Condition) -> bool {
        match &condition {
            Condition::Comparison { left, op, right } => {
                let left_value = row.get(left).unwrap().as_str();
                let right_value = right.as_str();
                match op {
                    BinaryOpCus::Lt => {
                        match self.col_map.get(left) {
                            None => { panic!("") }
                            Some(x) => {
                                match x {
                                    ColumnData::Int(_) => { left_value.parse::<i32>().unwrap() < right_value.parse::<i32>().unwrap() }
                                    ColumnData::Float(_) => { left_value.parse::<f32>().unwrap() < right_value.parse::<f32>().unwrap() }
                                    ColumnData::Str(_) => { left_value < right_value }
                                    ColumnData::Bool(_) => { left_value.parse::<bool>().unwrap() < right_value.parse::<bool>().unwrap() }
                                    ColumnData::None => { left_value < right_value }
                                }
                            }
                        }
                    }
                    BinaryOpCus::Gt => {
                        match self.col_map.get(left) {
                            None => { panic!("") }
                            Some(x) => {
                                match x {
                                    ColumnData::Int(_) => { left_value.parse::<i32>().unwrap() > right_value.parse::<i32>().unwrap() }
                                    ColumnData::Float(_) => { left_value.parse::<f32>().unwrap() > right_value.parse::<f32>().unwrap() }
                                    ColumnData::Str(_) => { left_value > right_value }
                                    ColumnData::Bool(_) => { left_value.parse::<bool>().unwrap() > right_value.parse::<bool>().unwrap() }
                                    ColumnData::None => { left_value > right_value }
                                }
                            }
                        }
                    }
                    BinaryOpCus::Eq => {
                        match self.col_map.get(left) {
                            None => { panic!("") }
                            Some(x) => {
                                match x {
                                    ColumnData::Int(_) => { left_value.parse::<i32>().unwrap() == right_value.parse::<i32>().unwrap() }
                                    ColumnData::Float(_) => { left_value.parse::<f32>().unwrap() == right_value.parse::<f32>().unwrap() }
                                    ColumnData::Str(_) => { left_value == right_value }
                                    ColumnData::Bool(_) => { left_value.parse::<bool>().unwrap() == right_value.parse::<bool>().unwrap() }
                                    ColumnData::None => { left_value == right_value }
                                }
                            }
                        }
                    }
                    _ => false,
                }
            }
            Condition::Logical { left, op, right } => {
                let left_result = self.evaluate_condition(row, &**left);
                let right_result = self.evaluate_condition(row, &**right);
                match op {
                    BinaryOpCus::And => left_result && right_result,
                    BinaryOpCus::Or => left_result || right_result,
                    _ => false,
                }
            }
        }
    }

    pub fn show_info(&self) {
        let mut headers = vec!["name", "datatype", "is_pk", "is_nullable", "default"];
        let rows = self.columns.iter().map(|c| c.attr()).collect::<Vec<HashMap<String, String>>>();
        let pt = PrettyTable::create(
            self.name.to_string(),
            headers.iter().map(|c| c.to_string()).collect::<Vec<String>>(),
            rows,
        );
        println!("{pt}")
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

pub struct PrettyTable {
    pub name: String,
    pub header: Vec<String>,
    pub values: HashMap<String, Vec<String>>,
    pub rows: Vec<HashMap<String, String>>,
}

impl PrettyTable {
    pub fn new(name: String, header: Vec<String>, values: HashMap<String, Vec<String>>) -> PrettyTable {
        return PrettyTable {
            name,
            header,
            values,
            rows: vec![],
        };
    }
    pub fn create(name: String, header: Vec<String>, rows: Vec<HashMap<String, String>>) -> PrettyTable {
        return PrettyTable {
            name,
            header,
            values: Default::default(),
            rows,
        };
    }
}

impl fmt::Display for PrettyTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut pt = PTable::new();
        let header_row = Row::new(
            self.header
                .iter()
                .map(|col| Cell::new(&col))
                .collect::<Vec<Cell>>(),
        );
        let mut pt_rows: Vec<Row> = vec![];
        if !self.rows.is_empty() {
            let num_rows = self.rows.len();
            pt_rows = vec![Row::new(vec![]); num_rows];
            for col in &self.header {
                for i in 0..num_rows {
                    pt_rows[i].add_cell(Cell::new(self.rows[i].get(col).unwrap()));
                }
            }
        } else {
            let num_rows = self.values.get(&self.header[0]).unwrap().len();
            pt_rows = vec![Row::new(vec![]); num_rows];
            for col in &self.header {
                let col_vals = self.values.get(col).unwrap();
                for i in 0..num_rows {
                    pt_rows[i].add_cell(Cell::new(&col_vals[i]));
                }
            }
        }
        pt.add_row(header_row);
        for row in pt_rows {
            pt.add_row(row);
        }
        write!(f, "{}", pt)
    }
}