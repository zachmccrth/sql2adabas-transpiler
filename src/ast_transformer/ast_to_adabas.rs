use std::string::ParseError;
use sqlparser::ast::Query;
use sqlparser::ast::Statement;
fn ast_to_adabas(ast: &Vec<Query>) -> Result<Vec<Statement>, ParseError> {
    ast.

}