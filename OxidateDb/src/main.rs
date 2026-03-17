use std::env;
use std::collections::HashMap;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::{SetExpr, Values};
mod store; // Import the store module

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: oxidated <sql>");
        std::process::exit(1);
    }

    let sql = &args[1];
    let dialect = PostgreSqlDialect {};

    let mut store = store::Store::new(); // Create a new store instance
    // Here we will parse the SQL and execute it against our store
    // Matche expression helps us handle both the success and error cases of parsing the SQL
    match Parser::parse_sql(&dialect, sql) { 
        // If parsing is successful, we get a vector of statements that we can iterate over and execute
        Ok(statements) => {
            for stmt in statements {
                match stmt {
                    // SELECT STATEMENT
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
                            // 4. call store to scan the table and print the results
                            let rows = store.scan_table(&table_name);
                            // if there are no rows, print a message saying so, otherwise print the rows
                            if rows.is_empty(){
                                println!("No rows found in table : {table_name}");
                            } else {
                                for row in rows {
                                    print!("{:?}", row);
                                }
                            }
                        }
                    }
                    // INSERT STATEMENT
                    sqlparser::ast::Statement::Insert (insert) => {
                        println!("Got an INSERT statement!");

                        // 2. Print the Table Name
                        let table_name = insert.table.to_string();
                        println!("Table Name: {}", table_name);

                        // 3. Print the Columns being Inserted into
                        let columns: Vec<String> = insert.columns.iter().map(|c| c.to_string()).collect();
                        println!("Columns: {}", columns.join(", "));
                        
                        // 4. Print the Values being Inserted
                        // if let Some(source) = insert.source{
                        //     if let SetExpr::Values(values) = *source.body{
                        //         for row in values.rows{
                        //             let vals: Vec<String> = row.iter().map(|v| v.to_string()).collect();
                        //             println!("Values: {}", vals.join(", "));
                        //         }
                        //     }
                        // }
                        // 5. call store to insert the values into the table
                        if let Some(source) = insert.source{
                            if let SetExpr::Values(values) = *source.body{
                                for row_values in values.rows{
                                    let mut row = store::Row::new();
                                    for (col, val) in columns.iter().zip(row_values.iter()) {
                                        row.insert(col.clone(), val.to_string());
                                        println!("  {} = {}", col, val);
                                    }
                                    store.insert_into_table(&table_name, row);
                                    println!("Inserted row into {table_name}");
                                }
                            }
                        }
                    }
                    // UPDATE STATEMENT
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
                    // DELETE STATEMENT
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
                    // CREATE TABLE STATEMENT
                    sqlparser::ast::Statement::CreateTable (create_table) => {
                        println!("Got a CREATE TABLE statement!");
                        
                        // 2. Print the Table Name
                        let table_name = create_table.name.to_string();
                        println!("Table Name: {}", table_name);

                        // 3. Print Columns and their Types
                        println!("Columns:");
                        for col in create_table.columns {
                            println!(" - Name: {}, Type: {}", col.name, col.data_type);
                        }
                        // 4. call store to create the table
                        store.create_table(&table_name);
                    }
                    other => {
                        eprintln!("Got someother statement: {other:#?}");
                    }
                }
            }
        }
        // If there was an error parsing the SQL, we print the error and exit with a non-zero status code
        Err(e) => {
            eprintln!("Parse error: {e}");
            std::process::exit(1);
        }
    }
}