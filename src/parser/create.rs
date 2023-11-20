use sqlparser::ast::{ColumnOption, DataType, Statement};
use sqlparser::dialect::AnsiDialect;
use sqlparser::parser::Parser;

//TODO Add Foreign Key
#[derive(Debug)]
pub struct ColumnAttr {
    pub name: String,
    pub datatype: String,
    pub is_pk: bool,
    pub is_nullable: bool,
    pub default: String,
}

pub struct CreateQuery {
    pub tb_name: String,
    pub cols: Vec<ColumnAttr>,
}

impl CreateQuery {
    pub fn new(sql: &str) -> Result<CreateQuery, String> {
        let dialect = AnsiDialect {};
        let binding = Parser::parse_sql(&dialect, &sql).unwrap();
        let statement: &Statement = binding.first().unwrap();
        println!("{:?}", statement);
        if let Statement::CreateTable {
            name,
            columns,
            constraints,
            ..
        } = statement {
            let tb_name = name.to_string();
            let mut cols: Vec<ColumnAttr> = vec![];
            for col in columns {
                let col_name = col.name.to_string();
                let data_type = match &col.data_type {
                    DataType::Char(_) => { "char" }
                    DataType::Decimal(_) => { "number" }
                    DataType::Float(_) => { "float" }
                    DataType::Int(_) => { "int" }
                    DataType::Double => { "float" }
                    DataType::Float(_) => { "float" }
                    DataType::Boolean => { "bool" }
                    DataType::Text => { "string" }
                    DataType::Varchar(_) => { "string" }
                    _ => {
                        "Error data type."
                    }
                };
                let mut is_pk = false;
                let mut is_nullable = true;
                let mut default: String = String::new();
                for opt in &col.options {
                    is_pk = match opt.option {
                        ColumnOption::Unique { is_primary } => { is_primary }
                        _ => { false }
                    };
                    is_nullable = match opt.option {
                        ColumnOption::NotNull => false,
                        _ => { false }
                    };
                    default = match &opt.option {
                        ColumnOption::Default(expr) => expr.to_string(),
                        _ => { String::new() }
                    };
                }
                println!("{col_name} {data_type} {is_pk} {is_nullable} {:?}", default);
                cols.push(ColumnAttr {
                    name: col_name,
                    datatype: data_type.to_string(),
                    is_pk,
                    is_nullable,
                    default,
                })
            }
            Ok(CreateQuery {
                tb_name,
                cols,
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
        name VARCHAR(100) NOT NULL DEFAULT \"\",
        role VARCHAR(100),
        department_id INT DEFAULT 0,
        abcd_id INT DEFAULT 0,
        email VARCHAR(100) UNIQUE,
        FOREIGN KEY (department_id) REFERENCES departments(id),
        FOREIGN KEY (abcd_id) REFERENCES abcds(id)
    );";
    let dialect = AnsiDialect {};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    match ast.first().unwrap() {
        Statement::CreateTable { .. } => {
            let create_query_result = CreateQuery::new(sql);
        }
        _ => {
            panic!("Unexpected statement type");
        }
    }
}