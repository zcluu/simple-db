use sqlparser::ast::{ColumnOption, DataType, Statement, TableConstraint};
use sqlparser::dialect::AnsiDialect;
use sqlparser::parser::Parser;

#[derive(Debug)]
pub struct ForeignKeyAttr {
    pub table: String,
    // current table's column
    pub col_a: String,
    // referred table's column
    pub col_b: String,
}

#[derive(Debug)]
pub struct ColumnAttr {
    pub name: String,
    pub datatype: String,
    pub is_pk: bool,
    pub is_nullable: bool,
    pub default: Option<String>,
}

#[derive(Debug)]
pub struct CreateQuery {
    pub tb_name: String,
    pub cols: Vec<ColumnAttr>,
    pub foreign_key: Vec<ForeignKeyAttr>,
}

impl CreateQuery {
    pub fn new(sql: &str) -> Result<CreateQuery, String> {
        let dialect = AnsiDialect {};
        let binding = Parser::parse_sql(&dialect, &sql).unwrap();
        let statement: &Statement = binding.first().unwrap();
        // println!("Create Statement:{:?}", statement);
        if let Statement::CreateTable {
            name,
            columns,
            constraints,
            ..
        } = statement
        {
            let tb_name = name.to_string();
            let mut curr_cols: Vec<String> = vec![];
            let mut cols: Vec<ColumnAttr> = vec![];
            let mut fkeys: Vec<ForeignKeyAttr> = vec![];
            for col in columns {
                let col_name = col.name.to_string();
                let data_type = match &col.data_type {
                    DataType::Char(_) => "char",
                    DataType::Float(_) => "float",
                    DataType::Int(_) => "int",
                    DataType::Double => "float",
                    DataType::Boolean => "bool",
                    DataType::Text => "string",
                    DataType::Varchar(_) => "string",
                    _ => "Error data type.",
                };
                let mut is_pk = false;
                let mut is_nullable = true;
                let mut default: Option<String> = None;
                for opt in &col.options {
                    is_pk = match opt.option {
                        ColumnOption::Unique { is_primary } => is_primary,
                        _ => false,
                    };
                    if is_pk {
                        is_nullable = false
                    } else {
                        is_nullable = match opt.option {
                            ColumnOption::NotNull => false,
                            _ => true,
                        };
                    }
                    default = match &opt.option {
                        ColumnOption::Default(expr) => Some(expr.to_string()),
                        _ => None,
                    };
                }
                // println!(
                //     "Column Attr:{col_name} {data_type} {is_pk} {is_nullable} {:?}",
                //     default
                // );
                curr_cols.push(col_name.to_string());
                cols.push(ColumnAttr {
                    name: col_name,
                    datatype: data_type.to_string(),
                    is_pk,
                    is_nullable,
                    default,
                })
            }
            for constraint in constraints {
                // println!("{:?}", constraint);
                if let TableConstraint::ForeignKey {
                    columns,
                    foreign_table,
                    referred_columns,
                    ..
                } = constraint
                {
                    let table = foreign_table.to_string();
                    let col_a = columns[0].value.to_string();
                    let col_b = referred_columns[0].value.to_string();
                    assert!(curr_cols.contains(&col_a));
                    fkeys.push(ForeignKeyAttr {
                        table,
                        col_a,
                        col_b,
                    });
                }
            }
            Ok(CreateQuery {
                tb_name,
                cols,
                foreign_key: fkeys,
            })
        } else {
            Err("Error".to_string())
        }
    }
}

#[test]
fn test_create_query_parsing() {
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
    let dialect = AnsiDialect {};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    match ast.first().unwrap() {
        Statement::CreateTable { .. } => {
            let create_query_result = CreateQuery::new(sql);
            let create_query = create_query_result.unwrap();
            assert_eq!(create_query.tb_name, "employees");
            let columns = create_query.cols;
            let fkeys = create_query.foreign_key;
            assert_eq!(columns.len(), 7);
            assert_eq!(columns[0].name, "id");
            assert_eq!(columns[0].datatype, "int");
            assert_eq!(columns[0].is_pk, true);
            assert_eq!(columns[0].is_nullable, false);

            assert_eq!(columns[1].name, "name");
            assert_eq!(columns[1].datatype, "string");
            assert_eq!(columns[1].is_pk, false);
            assert_eq!(columns[1].is_nullable, true);
            assert_eq!(columns[1].default, Some(String::from("Tom")));

            assert_eq!(columns[2].name, "role");
            assert_eq!(columns[2].datatype, "string");
            assert_eq!(columns[2].is_pk, false);
            assert_eq!(columns[2].is_nullable, true);

            assert_eq!(columns[3].name, "department_id");
            assert_eq!(columns[3].datatype, "int");
            assert_eq!(columns[3].is_pk, false);
            assert_eq!(columns[3].is_nullable, true);
            assert_eq!(columns[3].default, Some(String::from("0")));

            assert_eq!(columns[4].name, "abcd_id");
            assert_eq!(columns[4].datatype, "int");
            assert_eq!(columns[4].is_pk, false);
            assert_eq!(columns[4].is_nullable, true);
            assert_eq!(columns[4].default, Some(String::from("0")));

            assert_eq!(columns[5].name, "abcd_x");
            assert_eq!(columns[5].datatype, "int");
            assert_eq!(columns[5].is_pk, false);
            assert_eq!(columns[5].is_nullable, true);
            assert_eq!(columns[5].default, Some(String::from("0")));

            assert_eq!(columns[6].name, "email");
            assert_eq!(columns[6].datatype, "string");
            assert_eq!(columns[6].is_pk, false);
            assert_eq!(columns[6].is_nullable, true);
            assert_eq!(columns[6].default, None);

            assert_eq!(fkeys[0].table, "departments");
            assert_eq!(fkeys[0].col_a, "department_id");
            assert_eq!(fkeys[0].col_b, "id");

            assert_eq!(fkeys[1].table, "abcds");
            assert_eq!(fkeys[1].col_a, "abcd_id");
            assert_eq!(fkeys[1].col_b, "id");

            assert_eq!(fkeys[2].table, "abcds");
            assert_eq!(fkeys[2].col_a, "abcd_x");
            assert_eq!(fkeys[2].col_b, "x");
        }
        _ => {
            panic!("Unexpected statement type");
        }
    }
}
