use sqlparser::ast::{BinaryOperator, Expr, Query, SetExpr};
use sqlparser::parser::{ParserError};
use crate::ast_transformer::ast_to_adabas::BooleanExpr::Comparison;
use crate::ast_transformer::ast_to_adabas::LogicalOperator::AND;

pub fn ast_to_adabas(select_statement_ast: &Query) -> Result<Option<>, ParserError> {
    let where_clause = match select_statement_ast.body.as_ref() {
        SetExpr::Select(statement) => {
           &statement.selection
        }
        _ => {return Err(ParserError::ParserError("Not a select statement".to_string()))}
    };
    match where_clause {
        // This is a bit weird, but basically it might be possible to not have a where condition,
        // which means that it is a valid parse but there is nothing to return (therefore an
        // optional)
        None => {Ok(None)},
        Some(valid) => {match retrieve_tree(valid) {
            Ok(expression) => {Ok(Some(expression))}
            Err(err) => {Err(err)}
        }}
    }
}
// Ok what do we want to do here?
// I'd like to be able to define the tree. That means I need to know what a WHERE clause turns into
// Ok. After examination, the parser will literally accept any expression after a where, it would be
// my responsibility to ensure that only valid ADABAS expressions are accepted 
// (honestly a feature, thank you package authors)

// Here there is definitely and expression, just might be a parser error
fn retrieve_tree(expression: &Expr) -> Result<BooleanExpr , ParserError> {
    // Based on Rust docs, recursive types require a Box
    match expression {
        Expr::BinaryOp { op, left, right } => {
            let logical_op = match op {
                BinaryOperator::And => LogicOp{op: LogicalOperator::AND, left: Box::new(left), right: Box::new(right.clone())},
                BinaryOperator::Or => LogicOp{},
                BinaryOperator::Eq => Comparison()
                _ => {return Err(ParserError::ParserError("Not a logical operator".to_string()))}
            };

            Ok(BooleanExpr::LogicOp(Box::new(LogicOp {
                op: logical_op,
                left: Box::new(convert_expr_to_bool(*left)?),
                right: Box::new(convert_expr_to_bool(*right)?),
            })))
        }
    

        _ => Err("Not a valid boolean expression".into()),
    }
}

pub enum BooleanExpr {
    LogicOp(Box<LogicOp>),
    Comparison(Comparison),
}

pub struct LogicOp {
    pub left: Box<BooleanExpr>,
    pub right: Box<BooleanExpr>,
    pub op: LogicalOperator,
}

pub enum LogicalOperator {
    AND,
    OR,
}

pub struct Comparison {
    pub left: Identifier,
    pub right: Value,
    pub op: Comparator,
}

pub struct Identifier(pub String);

pub enum Comparator {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

