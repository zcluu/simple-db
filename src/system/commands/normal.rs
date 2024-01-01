use crate::database;
use crate::database::table::{PrettyTable, Table};
use crate::parser::create::CreateQuery;
use crate::parser::drop::DropQuery;
use crate::parser::insert::InsertQuery;
use crate::parser::join::FromType;
use crate::parser::select::SelectQuery;
use crate::parser::update::UpdateQuery;
use crate::parser::utils::parse_sql;
use std::collections::HashMap;

pub fn create_tb(query: String, db: &mut database::db::Database) {
    let state = parse_sql(query.as_str());
    let query = CreateQuery::format_stat(state).unwrap();
    let tb = Table::new(query);
    db.create_table(tb);
    db.save_disk().unwrap();
}

pub fn drop_tb(query: String, db: &mut database::db::Database) {
    let state = parse_sql(query.as_str());
    let query = DropQuery::format_stat(state);
    db.drop_table(query.drop_tbs);
    db.save_disk().unwrap()
}

pub fn insert_data(query: String, db: &mut database::db::Database) {
    let state = parse_sql(query.as_str());
    let query = InsertQuery::format_stat(state).unwrap();
    db.insert_row(query.tb_name, query.cols, query.rows);
    db.save_disk().unwrap();
}

pub fn select_data(query: String, db: &mut database::db::Database) {
    let state = parse_sql(query.as_str());
    let query = SelectQuery::format_stat(state).unwrap();
    let froms = query.from.clone();
    assert_eq!(froms.len(), 1);
    let from = froms.first().unwrap().to_owned();
    match from {
        FromType::Join { join_info, .. } => {
            let left_table = join_info.clone().left_table;
            let right_table = join_info.clone().right_table;
            let left_tb = db.get_table(left_table);
            let right_tb = db.get_table(right_table);
            let joint_tb = Table::join_tbs(left_tb, right_tb, join_info);
            joint_tb.select_data(query)
        }
        FromType::String { tb } => {
            let tb = db.get_table(tb);
            tb.select_data(query)
        }
    }
}

pub fn update_data(query: String, db: &mut database::db::Database) {
    let state = parse_sql(query.as_str());
    let query = UpdateQuery::format_stat(state);
    let tb: &mut Table = db.get_table_mut(query.tb_name);
    let rows = tb.get_rows();
    let filtered_rows = tb.filter_rows(&query.condition, rows.clone(), None);
    let pk = tb
        .columns
        .iter()
        .filter(|col| col.is_pk)
        .map(|col| col.name.clone())
        .collect::<Vec<String>>()
        .first()
        .unwrap()
        .to_string();
    let row_ids = filtered_rows
        .iter()
        .map(|row| row.get(pk.as_str()).unwrap().to_string())
        .collect::<Vec<String>>();
    let row_ixs: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter_map(|(ix, row)| {
            if row_ids.contains(row.get(pk.as_str()).unwrap()) {
                Some(ix)
            } else {
                None
            }
        })
        .collect();
    for (col, val) in &query.assignments {
        for &row_ix in &row_ixs {
            if let Some(column_data) = tb.col_map.get_mut(col.as_str()) {
                column_data.update_val(row_ix, val.clone());
            }
        }
    }
    db.save_disk().unwrap()
}

pub fn show_tb_data(query: String, db: &mut database::db::Database) {
    let vars = query.split(" ").collect::<Vec<&str>>();
    assert_eq!(vars.len(), 2);
    let tb_name = vars[1].to_string();
    let tb = db.get_table(tb_name);
    tb.print_table_data();
}

pub fn show_tb_info(query: String, db: &mut database::db::Database) {
    let vars = query.split(" ").collect::<Vec<&str>>();
    assert_eq!(vars.len(), 2);
    let tb_name = vars[1].to_string();
    let tb = db.get_table(tb_name);
    tb.show_info();
}

pub fn show_all_tbs(db: &mut database::db::Database) {
    let tb_names = db
        .tables
        .iter()
        .map(|x| {
            let mut table_map = HashMap::new();
            table_map.insert("Table Name".to_string(), x.name.clone());
            table_map
        })
        .collect::<Vec<HashMap<String, String>>>();
    let pt = PrettyTable::create(
        db.db_name.to_string(),
        vec!["Table Name".to_string()],
        tb_names,
    );
    println!("{pt}")
}
