mod ast_transformer;

use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
fn main() {
    let sql = "SELECT a, b, 123, myfunc(b) \
           FROM table_1 \
           WHERE a > b AND b < 100 \
           ORDER BY a DESC, b";

    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    
    let query = match ast.get(0).unwrap() {Statement::Query (q) => q.as_ref(),_ => panic!(),   };

    let expression_result = ast_transformer::ast_to_adabas::ast_to_adabas(&vec![query]);
    
    println!("{:?}", expression_result);
}
