use std::option::Option;
use sqlparser::ast::{BinaryOperator, Expr, SetExpr, Statement, TableFactor, Expr::{BinaryOp}};
use sqlparser::dialect::AnsiDialect;
use sqlparser::parser::Parser;

#[derive(Debug, PartialEq)]
pub enum BinaryOpCus {
    Lt,
    Gt,
    Eq,
    And,
    Or,
}

#[derive(Debug)]
pub enum Condition {
    Comparison {
        left: String,
        op: BinaryOpCus,
        right: String,
    },
    Logical {
        left: Box<Condition>,
        op: BinaryOpCus,
        right: Box<Condition>,
    },
}

#[derive(Debug)]
pub struct SelectQuery {
    pub from: String,
    pub projection: Vec<String>,
    pub condition: Option<Condition>,
}

impl SelectQuery {
    pub fn new(sql: &str) -> Result<SelectQuery, String> {
        let mut select_from: String = String::new();
        let mut select_projections: Vec<String> = vec![];
        let mut select_condition: Option<Condition> = None;

        let dialect = AnsiDialect {};
        let binding = Parser::parse_sql(&dialect, &sql).unwrap();
        let statement: &Statement = binding.first().unwrap();
        match statement {
            Statement::Query(bd) => {
                match &*bd.body {
                    SetExpr::Select(select) => {
                        let projects = &select.projection;
                        let froms = &select.from;
                        let exprs = &select.selection;
                        if !exprs.is_none() {
                            select_condition = Some(Condition::from_expr(&exprs.clone().unwrap()));
                        }
                        for f in froms {
                            if let TableFactor::Table { name, .. } = &f.relation { select_from = name.to_string(); }
                        }
                        for projection in projects {
                            let cname = projection.to_string();
                            select_projections.push(cname);
                        }
                    }
                    _ => {
                        return Err("Error".to_string());
                    }
                }
            }
            _ => {}
        }
        Ok(SelectQuery {
            from: select_from,
            projection: select_projections,
            condition: select_condition,
        })
    }
}

impl Condition {
    pub fn from_expr(expr: &Expr) -> Self {
        match expr {
            BinaryOp { left, op, right } => {
                let expr_op = match op {
                    BinaryOperator::Gt => BinaryOpCus::Gt,
                    BinaryOperator::Lt => BinaryOpCus::Lt,
                    BinaryOperator::Eq => BinaryOpCus::Eq,
                    BinaryOperator::And => BinaryOpCus::And,
                    BinaryOperator::Or => BinaryOpCus::Or,
                    _ => unimplemented!(),
                };
                if expr_op == BinaryOpCus::And || expr_op == BinaryOpCus::Or {
                    return Condition::Logical {
                        left: Box::new(Condition::from_expr(left)),
                        op: expr_op,
                        right: Box::new(Condition::from_expr(right)),
                    };
                } else {
                    return Condition::Comparison {
                        left: left.to_string(),
                        op: expr_op,
                        right: right.to_string(),
                    };
                }
            }
            _ => { panic!("Invalid operation.") }
        }
    }
}
