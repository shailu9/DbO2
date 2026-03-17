use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use sqlparser::ast:: Statement;


// This function will take a SQL string and parse it into a vector of Statements
pub fn parse_sql(sql: &str) -> Result<Vec<Statement>, String> {
    let dialect = PostgreSqlDialect {};
    Parser::parse_sql(&dialect, sql).map_err(|e| e.to_string())
}