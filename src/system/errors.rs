use std::fmt;
use std::fmt::{Formatter, Write};

#[derive(Debug)]
pub enum Errors {
    UnimplementedOperation,
    InvalidExpression,
    InvalidPattern,
    MissingLeftOperand,
    MissingRightOperand,
    UnsupportedDataType,
    ElementNotFound,
    DatabaseNotExisted,
    DiskSaveError,
    FileSystemError,
    ParseSQLError,
    InvalidCommand,
    TableNotExisted(String),
    TableExisted(String),
    InvalidColumnType,
}

impl Errors {
    pub fn print(self) {
        println!("{}", self);
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Errors::UnimplementedOperation => { f.write_str("This operation is unimplemented.") }
            Errors::InvalidExpression => { f.write_str("Expression is invalid.") }
            Errors::InvalidPattern => { f.write_str("InvalidPattern.") }
            Errors::MissingLeftOperand => { f.write_str("MissingLeftOperand.") }
            Errors::MissingRightOperand => { f.write_str("MissingRightOperand.") }
            Errors::UnsupportedDataType => { f.write_str("UnsupportedDataType.") }
            Errors::ElementNotFound => { f.write_str("ElementNotFound.") }
            Errors::DatabaseNotExisted => { f.write_str("DatabaseNotExisted.") }
            Errors::DiskSaveError => { f.write_str("DiskSaveError.") }
            Errors::FileSystemError => { f.write_str("FileSystemError.") }
            Errors::ParseSQLError => { f.write_str("ParseSQLError.") }
            Errors::InvalidCommand => { f.write_str("InvalidCommand.") }
            Errors::TableNotExisted(s) => { f.write_str(format!("Table {} is not existed.", s).as_str()) }
            Errors::TableExisted(s) => { f.write_str(format!("Table {} is existed.", s).as_str()) }
            Errors::InvalidColumnType => { f.write_str("InvalidColumnType") }
        }
    }
}