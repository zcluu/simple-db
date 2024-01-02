// parser.rs

use crate::parser::condition::Condition;
use crate::parser::join::FromType;
use crate::parser::utils::parse_sql;
use sqlparser::ast::Statement;

#[derive(Debug)]
pub struct DeleteQuery {
    pub tb_name: String,
    pub condition: Option<Condition>,
}

impl DeleteQuery {
    pub fn format_stat(state: Statement) -> DeleteQuery {
        let mut tb_name: String = "".to_string();
        let mut condition_data: Option<Condition> = None;
        if let Statement::Delete {
            from, selection, ..
        } = state
        {
            let from = FromType::new(from).first().unwrap().to_owned();
            if let FromType::String { tb } = from {
                tb_name = tb.to_string();
            }
            condition_data = Option::from(Condition::from_expr(&selection.unwrap()));
        }
        DeleteQuery {
            tb_name,
            condition: condition_data,
        }
    }
}

#[test]
pub fn test_delete() {
    let sql = "DELETE FROM users where id=1";
    let state = parse_sql(sql);
    println!("{:?}", state);
    DeleteQuery::format_stat(state);
}
