use sqlparser::ast::Statement;
use sqlparser::dialect::AnsiDialect;
use sqlparser::parser::Parser;

pub fn parse_sql(sql: &str) -> Statement {
    let dialect = AnsiDialect {};
    let binding = Parser::parse_sql(&dialect, &sql).unwrap();
    let statement: &Statement = binding.first().unwrap();
    statement.to_owned()
}

