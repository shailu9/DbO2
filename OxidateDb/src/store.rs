use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// A row is just a mapping of column names to values
pub type Row = HashMap<String, String>;

// A table is a collection of rows


// Store is a collection of tables, where each table is identified by its name
#[derive(Serialize, Deserialize)]
pub struct Store {
    tables: HashMap<String, Vec<Row>>,
    schemas: HashMap<String, Vec<String>>, // table name -> ordered column names
}

impl Store {
    // Create a new, empty store
    // In a real implementation, you might want to load existing data from disk here
    pub fn new() -> Self {
        Store {
            tables: HashMap::new(),
            schemas: HashMap::new(),
        }
    }

    // Create a new table with the given name
    // For simplicity, we are not defining columns and their types here, 
    // but in a real implementation, you would want to include that information as well.
    pub fn create_table(&mut self, table_name: &str, columns: Vec<String>) {
        let table_name = &table_name.to_lowercase(); // Normalize table name to lowercase
        self.tables.insert(table_name.to_string(), Vec::new());
        self.schemas.insert(table_name.to_string(), columns);
        println!("Created table: {}", table_name);
    }

    // Get the columns of a table
     pub fn get_columns(&self, table_name: &str) -> Option<&Vec<String>> {
        self.schemas.get(&table_name.to_lowercase())
    }
    // Scan a table and return all rows
    pub fn scan_table(&self, table_name: &str) -> Vec<Row> {
        let table_name = &table_name.to_lowercase();
        self.tables.get(table_name).cloned().unwrap_or_default()
    }

    // Scan a table with a filter condition and return matching rows
    pub fn scan_table_with_filter(&self , table_name : &str,col : &str, val : &str) -> Vec<Row> {
        let table_name = &table_name.to_lowercase();
        self.tables
            .get(table_name)
            .unwrap_or(&vec![])
            .iter()
            .filter(|row| row.get(col).map(|v| v == val).unwrap_or(false))
            .cloned()
            .collect()
    }

    // Insert a new row into a table
    pub fn insert_into_table(&mut self, table_name: &str, row: Row){
        let table_name = &table_name.to_lowercase(); // Normalize table name to lowercase
        match self.tables.get_mut(table_name) {
            Some(rows) => rows.push(row),
            None => println!("Error: table '{table_name}' does not exist"),
        }
    }

    // Load the store from disk (if it exists) or create a new one
    pub fn load() -> Self {
       match std::fs::read_to_string("oxidate.db.json") {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| {
                println!("Warning:currpted store file, starting with a new store");
                Store::new()
            }),
            Err(_) => Store::new(),           
       }
    }

    // Save the store to disk
    pub fn save(&self) {
        let json = serde_json :: to_string_pretty(self).unwrap();
        std::fs::write("oxidate.db.json", json).unwrap();
        println!("Store saved to disk.");
    }

    // Delete rows from a table that match a filter condition
    pub fn delete_from_table_with_filter(&mut self, table_name:&str, col:&str, val:&str){
        let table_name = table_name.to_lowercase();
        if let Some(rows) = self.tables.get_mut(&table_name){
            rows.retain(|row| row.get(col).map(|v|v != val).unwrap_or(true));
            println!("Deleted rows from table '{table_name}' where {col} = {val}");
        } else {
            println!("Error: table '{table_name}' does not exist");
        }
    }

    // Update rows in a table that match a filter condition
    pub fn update_table_with_filter(&mut self, table_name:&str, col : &str, val: &str, filter_col : &str, filter_val : &str){
        let table_name = table_name.to_lowercase();
        if let Some(rows) = self.tables.get_mut(&table_name) {
            let mut updated_count = 0;
            for row in rows.iter_mut() {
                if row.get(filter_col).map(|v| v == filter_val).unwrap_or(false) {
                    row.insert(col.to_string(), val.to_string());
                    updated_count += 1;
                }
            }
            println!("Updated {updated_count} rows in table '{table_name}'");
        } else {
            println!("Error: table '{table_name}' does not exist");
        }
    }
}