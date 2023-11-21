use sqlparser::ast::{Expr, Query, SetExpr, Statement, Value, Values};
use sqlparser::dialect::AnsiDialect;
use sqlparser::parser::Parser;

#[derive(Debug)]
pub struct InsertQuery {
    pub tb_name: String,
    pub cols: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl InsertQuery {
    pub fn new(sql: &str) -> Result<InsertQuery, String> {
        let dialect = AnsiDialect {};
        let binding = Parser::parse_sql(&dialect, &sql).unwrap();
        let statement: &Statement = binding.first().unwrap();
        let tb_name: Option<String>;
        let mut cols_data: Vec<String> = vec![];
        let mut rows_data: Vec<Vec<String>> = vec![];
        if let Statement::Insert {
            table_name,
            columns,
            source,
            ..
        } = statement
        {
            tb_name = Some(table_name.to_string());
            for col in columns {
                cols_data.push(col.value.to_string());
            }
            match &**source {
                Query { body, .. } => {
                    // println!("{body}");

                    if let SetExpr::Values(Values {
                        explicit_row: _explicit_row,
                        rows,
                    }) = &**body
                    {
                        for col_its in rows {
                            let mut row_vals: Vec<String> = vec![];
                            for it in col_its {
                                match it {
                                    Expr::Value(v) => match v {
                                        Value::Number(x, _) => {
                                            row_vals.push(x.to_string());
                                        }
                                        Value::Boolean(x) => {
                                            row_vals.push(x.to_string());
                                        }
                                        Value::SingleQuotedString(x) => {
                                            row_vals.push(x.to_string());
                                        }
                                        Value::Null => {
                                            row_vals.push("NULL".to_string());
                                        }
                                        _ => return Err(String::from("Invalid type.")),
                                    },
                                    _ => return Err(String::from("Invalid operation.")),
                                }
                            }
                            rows_data.push(row_vals);
                        }
                    } else {
                        return Err(String::from("Invalid operation."));
                    }
                }
            }
        } else {
            return Err(String::from("Invalid operation."));
        }
        match tb_name {
            None => {
                return Err(String::from("Invalid operation."));
            }
            Some(_) => Ok(InsertQuery {
                tb_name: tb_name.unwrap(),
                cols: cols_data,
                rows: rows_data,
            }),
        }
    }
}

#[test]
fn test_insert_query_parsing() {
    let sql =
        "INSERT INTO example_table (id, name, age) VALUES (1, 'John Doe', '25'),(2, 'Tom', '30');";
    let dialect = AnsiDialect {};
    let ast = sqlparser::parser::Parser::parse_sql(&dialect, sql).unwrap();
    match ast.first().unwrap() {
        Statement::Insert { .. } => {
            let rows_result = vec![vec!["1", "John Doe", "25"], vec!["2", "Tom", "30"]];
            let insert_query = InsertQuery::new(sql).unwrap();
            println!("{:?}", insert_query);
            assert_eq!("example_table", insert_query.tb_name);
            assert_eq!(vec!["id", "name", "age"], insert_query.cols);
            assert_eq!(rows_result, insert_query.rows);
        }
        _ => panic!("Parsing failed"),
    }
}
