use crate::store::{self, Store};
use sqlparser::ast::{SetExpr, Statement};

// This function will take a vector of Statements and execute them against the Store
pub fn execute_statement(stmt: Statement, store: &mut Store) -> String {
    match stmt {
        // SELECT STATEMENT
        sqlparser::ast::Statement::Query(query) => {
            println!("Got a SELECT query!");
            let mut output = String::new();
            if let SetExpr::Select(select) = *query.body {
                //  Table name
                let table_name = select
                    .from
                    .first()
                    .map(|table_with_joins| &table_with_joins.relation)
                    .unwrap()
                    .to_string()
                    .to_lowercase();

                // columns
                let columns: Vec<String> =
                    select.projection.iter().map(|p| p.to_string()).collect();

                // filter
                let filter = match select.selection {
                    Some(expr) => expr.to_string(),
                    None => "none".into(),
                };

                println!(
                    "Table: {}, Columns: {}, Filter: {}",
                    table_name,
                    columns.join(", "),
                    filter
                );
                // 4. call store to scan the table and print the results
                // expr.to_string() gives "id = 1" — we'll parse it properly later
                // for now, naive split on " = "
                let rows: Vec<store::Row>;
                let parts: Vec<&str> = filter.splitn(2, " = ").collect();
                if parts.len() == 2 {
                    rows = store.scan_table_with_filter(&table_name, parts[0], parts[1]);
                } else {
                    rows = store.scan_table(&table_name);
                }

                // if there are no rows, print a message saying so, otherwise print the rows
                if rows.is_empty() {
                    output.push_str(&format!("Returned {} rows", rows.len()));
                } else {
                    let is_select_all = columns.iter().any(|c| c == "*");
                    for row in rows {
                        if is_select_all {
                            output.push_str(&format!("{:?}\n", row));
                        } else {
                            let filtered: std::collections::HashMap<String, String> = row
                                .into_iter()
                                .filter(|(k, _)| columns.iter().any(|c| c.to_lowercase() == *k))
                                .collect();
                            output.push_str(&format!("{:?}\n", filtered));
                        }
                    }
                }
            }
            output
        }
        // INSERT STATEMENT
        sqlparser::ast::Statement::Insert(insert) => {
            println!("Got an INSERT statement!");

            // 2. Print the Table Name
            let table_name = insert.table.to_string().to_lowercase();
            println!("Table Name: {}", table_name);

            // 3. Print the Columns being Inserted into
            let columns: Vec<String> = if insert.columns.is_empty() {
                store.get_columns(&table_name).cloned().unwrap_or_else(|| {
                    let count = if let Some(src) = &insert.source {
                        if let SetExpr::Values(v) = src.body.as_ref() {
                            v.rows.first().map(|r| r.len()).unwrap_or(0)
                        } else {
                            0
                        }
                    } else {
                        0
                    };
                    (0..count).map(|i| format!("col{i}")).collect()
                })
            } else {
                insert.columns.iter().map(|c| c.to_string()).collect()
            };

            println!("Columns: {}", columns.join(", "));

            // 4. Print the Values being Inserted
            // 5. call store to insert the values into the table
            if let Some(source) = insert.source {
                if let SetExpr::Values(values) = *source.body {
                    for row_values in values.rows {
                        let mut row = store::Row::new();
                        for (col, val) in columns.iter().zip(row_values.iter()) {
                            let val_str = val.to_string();
                            if val_str.starts_with('"') && val_str.ends_with('"') {
                                return format!("Error: double quotes are for identifiers, use single quotes for strings");
                            }
                            let clean_val = val_str.trim_matches('\'').to_string();
                            println!("  {} = {}", col, clean_val);
                            row.insert(col.clone(), clean_val);
                        }
                        store.insert_into_table(&table_name, row);
                        println!("Inserted row into {table_name}");
                    }
                }
            }
            store.save();
            format!("Values inserted into table: {}", table_name)
        }
        // UPDATE STATEMENT
        sqlparser::ast::Statement::Update(update) => {
            println!("Got an UPDATE statement!");
            // 2. Print the Table Name
            let table_name = update.table.to_string().to_lowercase();
            println!("Table Name: {}", table_name);

            // 3. Print the Columns being Updated
            println!(
                "Columns: {}",
                update
                    .assignments
                    .iter()
                    .map(|a| format!("{} = {}", a.target, a.value))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            // filter
            let filter = match update.selection {
                Some(expr) => expr.to_string(),
                None => "none".into(),
            };
            print!("Filter: {}", filter);
            let filter_parts: Vec<&str> = filter.splitn(2, " = ").collect();

            for assignment in update.assignments {
                let col = assignment.target.to_string();
                let val_str = assignment.value.to_string();
                if val_str.starts_with('"') && val_str.ends_with('"') {
                    return format!(
                        "Error: double quotes are for identifiers, use single quotes for strings"
                    );
                }
                let val = val_str.trim_matches('\'').to_string();

                if filter_parts.len() == 2 {
                    store.update_table_with_filter(
                        &table_name,
                        &col,
                        &val,
                        filter_parts[0].trim(),
                        filter_parts[1].trim(),
                    );
                } else {
                    println!("No valid filter found, skipping update for column {col}");
                }
            }
            store.save();
            format!("Table updated: {}", table_name)
        }
        // DELETE STATEMENT
        sqlparser::ast::Statement::Delete(delete) => {
            println!("Got a DELETE statement!");
            // 2. Print the Table Name
            //let tabel_name = delete.name.to_string();
            //println!("Table Name: {}", tabel_name);
            let tabel_name =
                if let sqlparser::ast::FromTable::WithFromKeyword(tables) = &delete.from {
                    tables
                        .first()
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
            let parts: Vec<&str> = filter.splitn(2, " = ").collect();
            if parts.len() == 2 {
                store.delete_from_table_with_filter(&tabel_name, &parts[0], &parts[1]);
            } else {
                println!("No valid filter found, skipping delete");
            }
            store.save();
            format!("Rows deleted from table: {}", tabel_name)
        }
        // CREATE TABLE STATEMENT
        sqlparser::ast::Statement::CreateTable(create_table) => {
            println!("Got a CREATE TABLE statement!");

            // 2. Print the Table Name
            let table_name = create_table.name.to_string();
            println!("Table Name: {}", table_name);

            // 3. Print Columns and their Types
            let columns: Vec<String> = create_table
                .columns
                .iter()
                .map(|c| {
                    println!("  Column: {}, Type: {}", c.name, c.data_type);
                    c.name.to_string()
                })
                .collect();
            //println!("Columns:{columns:?}");
            // 4. call store to create the table
            store.create_table(&table_name, columns);
            store.save();
            format!("Table created: {}", table_name)
        }
        other => format!("Unsupported: {other}\n"),
    }
}
