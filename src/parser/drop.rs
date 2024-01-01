use sqlparser::ast::Statement;
use crate::parser::utils::parse_sql;

pub struct DropQuery {
    pub drop_tbs: Vec<String>,
}

impl DropQuery {
    pub fn format_stat(state: Statement) -> DropQuery {
        if let Statement::Drop { names, .. } = state {
            let drop_tbs = names.iter().map(|x| x.to_string()).collect::<Vec<String>>();
            return DropQuery { drop_tbs };
        } else {
            panic!("Invalid query.")
        }
    }
}

#[test]
fn test_drop_query() {
    let sql = "DROP TABLE articles;";
    let state = parse_sql(sql);
    println!("{:?}", state);
    DropQuery::format_stat(state);
}
