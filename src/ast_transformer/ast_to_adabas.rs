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
// Ok what do we want to do here?
// I'd like to be able to define the tree. That   means I need to know what a WHERE clause turns into
// Ok. After examination, the parser will literally accept any expression after a where, it would be my responsibility to ensure
// that only valid ADABAS expressions are accepted (honestly a feature, thank you package authors)
fn retrieve_tree(expression: &Expr) -> Result<Expr, ParserError> {
    match  expression {
        Expr::BinaryOp { op, left, right } => {
           Ok(Expr::BinaryOp {op: op.clone(), left: left.clone(), right: right.clone()}) 
        }
        _ => Err(ParserError::ParserError("Not a binary query".to_string()))
    } 
}