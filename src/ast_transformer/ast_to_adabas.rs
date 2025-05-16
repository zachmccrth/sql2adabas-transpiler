use sqlparser::ast::{Expr, Query, SetExpr};
use sqlparser::parser::{ParserError};

pub fn ast_to_adabas(select_statement_ast: &Query) -> Result<Option<Expr>, ParserError> {
    let where_clause = match select_statement_ast.body.as_ref() {
        SetExpr::Select(statement) => {
           &statement.selection
        }
        _ => {return Err(ParserError::ParserError(format!("Not a select statement")))} 
    };
    match where_clause {
        // This is a bit weird, but basically it might be possible to not have a where condition,
        // which means that it is a valid parse but there is nothing to return (therefore an 
        // optional)
        None => {Ok(None)},
        Some(valid) => {match retrieve_tree(valid) {
            Ok(expression) => {Ok(Some(expression))}
            Err(errortype) => {Err(errortype)}
        }}
    }
}
// Ok what do we want to do here?
// I'd like to be able to define the tree. That means I need to know what a WHERE clause turns into
// Ok. After examination, the parser will literally accept any expression after a where, it would be
// my responsibility to ensure that only valid ADABAS expressions are accepted 
// (honestly a feature, thank you package authors)

// Here there is definitely and expression, just might be a parser error
fn retrieve_tree(expression: &Expr) -> Result<Expr, ParserError> {
    // Based on Rust docs, recursive types require a Box
    match  expression {
        Expr::BinaryOp { op, left, right } => {
           Ok(Expr::BinaryOp {op: op.clone(), left: left.clone(), right: right.clone()}) 
        }
        _ => Err(ParserError::ParserError("Not a binary query".to_string()))
    } 
}