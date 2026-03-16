use std::env;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: oxidated <sql>");
        std::process::exit(1);
    }

    let sql = &args[1];
    let dialect = PostgreSqlDialect {};

    match Parser::parse_sql(&dialect, sql) {
        Ok(statements) => {
            println!("Parsed {} statement(s):", statements.len());
            for stmt in statements {
                println!("{stmt:#?}");
            }
        }
        Err(e) => {
            eprintln!("Parse error: {e}");
            std::process::exit(1);
        }
    }
}