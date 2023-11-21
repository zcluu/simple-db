use sqlparser::ast::{ColumnOption, DataType, Statement, TableConstraint, Select, SelectItem, SetExpr, TableFactor, Expr, BinaryOperator};
use sqlparser::ast::KillType::Query;
use sqlparser::dialect::AnsiDialect;
use sqlparser::parser::Parser;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum BinaryOpCus {
    // <
    Lt,
    // >
    Gt,
    // =
    Eq,
}

// Define Where Expression
#[derive(Debug, PartialEq)]
pub struct WhereExpr {
    // field
    pub left: String,
    // < = >
    pub op: BinaryOpCus,
    // value
    pub right: String,
}

#[derive(Debug)]
pub struct SelectQuery {
    pub from: String,
    pub projection: Vec<String>,
    pub selection: Vec<WhereExpr>,
}

impl SelectQuery {
    pub fn new(sql: &str) -> Result<SelectQuery, String> {
        let mut select_from: String = String::new();
        let mut select_projections: Vec<String> = vec![];
        let mut select_selections: Vec<WhereExpr> = vec![];

        let dialect = AnsiDialect {};
        let binding = Parser::parse_sql(&dialect, &sql).unwrap();
        let statement: &Statement = binding.first().unwrap();
        println!("Select Statement:{:?}", statement);
        match statement {
            Statement::Query(bd) => {
                // println!("{:?}",bd.body);
                match &*bd.body {
                    SetExpr::Select(select) => {
                        println!("{:?}", select);
                        let projects = &select.projection;
                        let froms = &select.from;
                        let exprs = &select.selection;
                        for f in froms {
                            if let TableFactor::Table {
                                name,
                                alias,
                                args,
                                with_hints,
                                version,
                                partitions
                            } = &f.relation {
                                println!("Table Name: {name}");
                                select_from = name.to_string();
                            }
                        }

                        for projection in projects {
                            // println!("{:?}", projection.to_string());
                            select_projections.push(projection.to_string());
                        }
                        for expr in exprs {
                            println!("{:?}", expr);
                            if let Expr::BinaryOp {
                                left, op, right
                            } = expr {
                                let expr_op = match op {
                                    BinaryOperator::Gt => { BinaryOpCus::Gt }
                                    BinaryOperator::Lt => { BinaryOpCus::Lt }
                                    BinaryOperator::Eq => { BinaryOpCus::Eq }
                                    _ => {
                                        BinaryOpCus::Eq
                                    }
                                };
                                println!("{left} {:?} {right}", expr_op);
                                select_selections.push(WhereExpr {
                                    left: left.to_string(),
                                    op: expr_op,
                                    right: right.to_string(),
                                });
                            }
                        }
                    }
                    _ => {
                        return Err("Error".to_string());
                    }
                }
            }
            _ => {}
        }
        println!("{:?}", select_from);
        println!("{:?}", select_projections);
        println!("{:?}", select_selections);
        Ok(SelectQuery {
            from: select_from,
            projection: select_projections,
            selection: select_selections,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_query_new() {
        let sql = "SELECT id,name FROM employees WHERE id > 1";
        let select_query = SelectQuery::new(sql).unwrap();
        println!("{:?}", select_query);
        assert_eq!(select_query.from, "employees");
        assert_eq!(select_query.projection, vec!["id", "name"]);
        assert_eq!(
            select_query.selection,
            vec![WhereExpr {
                left: "id".to_string(),
                right: "1".to_string(),
                op: BinaryOpCus::Gt,
            }]
        );
    }

    // #[test]
    // fn test_insert_projections() {
    //     let mut select_query = SelectQuery {
    //         from: "employees".to_string(),
    //         projection: vec!["id".to_string()],
    //         where_expressions: vec![],
    //     };
    //
    //     select_query.insert_projections(vec!["name".to_string(), "email".to_string()]);
    //     assert_eq!(select_query.projection, vec!["name", "email"]);
    // }
}
