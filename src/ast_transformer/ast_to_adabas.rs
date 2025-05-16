use sqlparser::ast::{Expr, Query, SetExpr};
use sqlparser::parser::{ParserError};

pub fn ast_to_adabas(ast: &Vec<&Query>) -> Result<Expr, ParserError> {
    let thing: &Query = ast.get(0).unwrap();
    let select = match thing.body.as_ref() {
        SetExpr::Select(statement) => {
           &statement.selection
        }
        _ => {panic!("Not a select!")} 
    };
    match select {
        None => {Err(ParserError::ParserError(format!("No select!")))},
        Some(valid) => {retrieve_tree(valid)}
    }
}

fn retrieve_tree(expression: &Expr) -> Result<Expr, ParserError> {
    match  expression {
        Expr::BinaryOp { op, left, right } => {
           return Ok(Expr::BinaryOp {op: op.clone(), left: left.clone(), right: right.clone()}) 
        }
        _ => return Err(ParserError::ParserError("Not a binary query".to_string()))
    } 
}