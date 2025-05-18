use sqlparser::ast;
use sqlparser::ast::{BinaryOperator, Expr, Query, SetExpr};
use sqlparser::parser::ParserError;
use std::fmt::Display;

pub fn ast_to_adabas(select_statement_ast: &Query) -> Result<Option<BooleanExpr>, ParserError> {
    let where_clause = match select_statement_ast.body.as_ref() {
        SetExpr::Select(statement) => &statement.selection,
        _ => {
            return Err(ParserError::ParserError(
                "Not a select statement".to_string(),
            ));
        }
    };
    match where_clause {
        // This is a bit weird, but basically it might be possible to not have a where condition,
        // which means that it is a valid parse but there is nothing to return (therefore an
        // optional)
        None => Ok(None),
        Some(valid) => match retrieve_tree(valid) {
            Ok(expression) => Ok(Some(expression)),
            Err(err) => Err(err),
        },
    }
}
// Ok what do we want to do here?
// I'd like to be able to define the tree. That means I need to know what a WHERE clause turns into
// Ok. After examination, the parser will literally accept any expression after a where, it would be
// my responsibility to ensure that only valid ADABAS expressions are accepted
// (honestly a feature, thank you package authors)

// Here there is definitely an expression, just might be a parser error
fn retrieve_tree(expression: &Expr) -> Result<BooleanExpr, ParserError> {
    let Expr::BinaryOp { op, left, right } = expression else {
        return Err(ParserError::ParserError(
            "Not a binary expression".to_string(),
        ));
    };

    match op {
        BinaryOperator::And | BinaryOperator::Or => {
            let left_expr = Box::new(retrieve_tree(left)?);
            let right_expr = Box::new(retrieve_tree(right)?);
            let logical_op = match op {
                BinaryOperator::And => LogicalOperator::AND,
                BinaryOperator::Or => LogicalOperator::OR,
                _ => unreachable!(),
            };

            Ok(BooleanExpr::LogicOp(Box::new(LogicOp {
                op: logical_op,
                left: left_expr,
                right: right_expr,
            })))
        }

        BinaryOperator::Eq
        | BinaryOperator::NotEq
        | BinaryOperator::Lt
        | BinaryOperator::Gt
        | BinaryOperator::LtEq
        | BinaryOperator::GtEq => {
            let ident = extract_identifier(&**left)?;
            let value = extract_value(right)?;
            let comparator = match op {
                BinaryOperator::Eq => Comparator::Equal,
                BinaryOperator::NotEq => Comparator::NotEqual,
                BinaryOperator::Lt => Comparator::LessThan,
                BinaryOperator::Gt => Comparator::GreaterThan,
                BinaryOperator::LtEq => Comparator::LessThanOrEqual,
                BinaryOperator::GtEq => Comparator::GreaterThanOrEqual,
                _ => unreachable!(),
            };

            Ok(BooleanExpr::Comparison(Comparison {
                op: comparator,
                left: ident,
                right: value,
            }))
        }
        _ => Err(ParserError::ParserError(format!(
            "Unhandled operator: {:?}",
            op
        ))),
    }
}

fn extract_value(value: &Expr) -> Result<Value, ParserError> {
    let Expr::Value(value) = value else {
        return Err(ParserError::ParserError(format!(
            "Not a literal value: {}",
            value
        )));
    };
    match &value.value {
        ast::Value::Boolean(b) => Ok(Value::Bool(*b)),
        ast::Value::Number(n, _) => Ok(Value::Number(n.to_string())),
        ast::Value::SingleQuotedString(s)
        | ast::Value::DoubleQuotedString(s)
        | ast::Value::TripleDoubleQuotedString(s) => Ok(Value::String(s.clone())),
        _ => Err(ParserError::ParserError("Not a valid value".to_string())),
    }
}

fn extract_identifier(expr: &Expr) -> Result<Identifier, ParserError> {
    let Expr::Identifier(ident) = expr else {
        return Err(ParserError::ParserError("Not a identifier".to_string()));
    };
    Ok(Identifier {
        value: ident.value.clone(),
    })
}
#[derive(Debug)]
pub enum BooleanExpr {
    LogicOp(Box<LogicOp>),
    Comparison(Comparison),
}
impl Display for BooleanExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BooleanExpr::LogicOp(op) => write!(f, "{:?}", op),
            BooleanExpr::Comparison(comparison) => write!(f, "{:?}", &comparison),
        }
    }
}

impl Display for LogicOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug)]
pub struct LogicOp {
    pub op: LogicalOperator,
    pub left: Box<BooleanExpr>,
    pub right: Box<BooleanExpr>,
}

#[derive(Debug)]
pub enum LogicalOperator {
    AND,
    OR,
}

#[derive(Debug)]
pub struct Comparison {
    pub op: Comparator,
    pub left: Identifier,
    pub right: Value,
}

#[derive(Debug)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug)]
pub enum Comparator {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

#[derive(Debug)]
pub enum Value {
    Number(String),
    Bool(bool),
    String(String),
}
