use std::env;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::{SetExpr};

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
            for stmt in statements {
                match stmt {
                    sqlparser::ast::Statement::Query(query) => {
                        println!("Got a SELECT query!");
                        if let SetExpr::Select(select) = *query.body {

                            //  Table name
                            let table_name = select
                                                .from
                                                .first()
                                                .map(|table_with_joins| &table_with_joins.relation)
                                                .unwrap()
                                                .to_string();

                            // columns 
                            let columns: Vec<String> = select
                                                        .projection
                                                        .iter()
                                                        .map(|p|p.to_string())
                                                        .collect();

                            // filter
                             let filter = match select.selection {
                                Some(expr) => expr.to_string(),
                                None => "none".into(),
                            };
                            
                            println!(
                            "Table: {}, Columns: {}, Filter: {}",
                            table_name,
                            columns.join(", "),
                            filter);
                        }
                    }
                    // match other statement types here (INSERT, UPDATE, etc.)
                    sqlparser::ast::Statement::Insert (insert) => {
                        println!("Got an INSERT statement!");

                        // 2. Print the Table Name
                        let tabel_name = insert.table.to_string();
                        println!("Table Name: {}", tabel_name);

                        // 3. Print the Columns being Inserted into
                        println!("Columns: {}", insert.columns.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", "));
                        
                        // 4. Print the Values being Inserted
                        if let Some(source) = insert.source{
                            if let SetExpr::Values(values) = *source.body{
                                for row in values.rows{
                                    let vals: Vec<String> = row.iter().map(|v| v.to_string()).collect();
                                    println!("Values: {}", vals.join(", "));
                                }
                            }
                        }
                    }
                    sqlparser::ast::Statement::Update (update) => {
                        println!("Got an UPDATE statement!");
                        // 2. Print the Table Name
                        let tabel_name = update.table.to_string();
                        println!("Table Name: {}", tabel_name);

                        // 3. Print the Columns being Updated
                        println!("Columns: {}", update.assignments.iter().map(|a| format!("{} = {}", a.target, a.value)).collect::<Vec<String>>().join(", "));
                        // filter
                        let filter = match update.selection {
                                        Some(expr) => expr.to_string(),
                                        None => "none".into(),
                                    };
                        print!("Filter: {}", filter);
                    }
                    sqlparser::ast::Statement::Delete (delete) => {
                        println!("Got a DELETE statement!");
                        // 2. Print the Table Name
                        //let tabel_name = delete.name.to_string();
                        //println!("Table Name: {}", tabel_name);
                        let tabel_name = if let sqlparser::ast::FromTable::WithFromKeyword(tables) = &delete.from{
                            tables.first()
                                    .map(|t| t.relation.to_string())
                                    .unwrap_or("unknown".into())
                        } else {
                            "unknown".into()
                        };
                        println!("Table Name: {}", tabel_name);
                        // 3. Print the Filter Condition
                        let filter = match delete.selection {
                            Some(expr) => expr.to_string(),
                            None => "none".into(),
                        };
                        println!("Filter: {}", filter);
                    }
                    sqlparser::ast::Statement::CreateTable (create_table) => {
                        println!("Got a CREATE TABLE statement!");
                        
                        // 2. Print the Table Name
                        println!("Table Name: {}", create_table.name);

                        // 3. Print Columns and their Types
                        println!("Columns:");
                        for col in create_table.columns {
                            println!(" - Name: {}, Type: {}", col.name, col.data_type);
                        }
                        // 4. Print values of the columns
                    }
                    other => {
                        eprintln!("Got someother statement: {other:#?}");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Parse error: {e}");
            std::process::exit(1);
        }
    }
}