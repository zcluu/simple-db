use sqlparser::ast::{BinaryOperator, Expr};
use sqlparser::ast::Expr::{BinaryOp, IsNull, Like};
use crate::parser::select::BinaryOpCus;

#[derive(Debug)]
pub enum Condition {
    Comparison {
        left: String,
        op: BinaryOpCus,
        right: Option<String>,
    },
    Logical {
        left: Box<Condition>,
        op: BinaryOpCus,
        right: Box<Condition>,
    },
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
                        right: Option::from(right.to_string()),
                    };
                }
            }
            IsNull(x) => Condition::Comparison {
                left: x.to_string(),
                op: BinaryOpCus::IsNull,
                right: None,
            },
            Like { expr, pattern, .. } => Condition::Comparison {
                left: expr.to_string(),
                op: BinaryOpCus::Like,
                right: Option::from(pattern.to_string()),
            },
            _ => {
                panic!("Invalid operation.")
            }
        }
    }
}
