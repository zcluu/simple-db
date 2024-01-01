use crate::parser::join::FromType;
use sqlparser::ast::{
    SetExpr, Statement,
};
use std::option::Option;
use crate::parser::condition::Condition;
use crate::parser::utils::parse_sql;

#[derive(Debug, PartialEq)]
pub enum BinaryOpCus {
    Lt,
    Gt,
    Eq,
    And,
    Or,
    IsNull,
    IsTrue,
    Like,
}

#[derive(Debug)]
pub struct SelectQuery {
    pub from: Vec<FromType>,
    pub projection: Vec<String>,
    pub condition: Option<Condition>,
}

impl SelectQuery {
    pub fn format_stat(statement: Statement) -> Result<SelectQuery, String> {
        let mut select_from: Vec<FromType> = vec![];
        let mut select_projections: Vec<String> = vec![];
        let mut select_condition: Option<Condition> = None;
        match statement {
            Statement::Query(bd) => match &*bd.body {
                SetExpr::Select(select) => {
                    let projects = &select.projection;
                    let froms = &select.from;
                    let exprs = &select.selection;
                    if !exprs.is_none() {
                        select_condition = Some(Condition::from_expr(&exprs.clone().unwrap()));
                    }
                    select_from = FromType::new(froms.to_owned());
                    for projection in projects {
                        let cname = projection.to_string();
                        select_projections.push(cname);
                    }
                }
                _ => {
                    return Err("Error".to_string());
                }
            },
            _ => {}
        }
        Ok(SelectQuery {
            from: select_from,
            projection: select_projections,
            condition: select_condition,
        })
    }
}


#[test]
pub fn test_select() {
    let sql = "SELECT articles.id, articles.title, articles.userid, users.username FROM articles JOIN users ON articles.userid = users.id;";
    let stat = parse_sql(sql);
    let query = SelectQuery::format_stat(stat);

    let sql2 = "SELECT id,username from users;";
    let stat2 = parse_sql(sql2);
    let query2 = SelectQuery::format_stat(stat2);
}
