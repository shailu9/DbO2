use std::collections::HashMap;

// A row is just a mapping of column names to values
pub type Row = HashMap<String, String>;

// A table is a collection of rows


// Store is a collection of tables, where each table is identified by its name
pub struct Store {
    tables: HashMap<String, Vec<Row>>,
}

impl Store {
    // Create a new, empty store
    // In a real implementation, you might want to load existing data from disk here
    pub fn new() -> Self {
        Store {
            tables: HashMap::new(),
        }
    }

    // Create a new table with the given name
    // For simplicity, we are not defining columns and their types here, 
    // but in a real implementation, you would want to include that information as well.
    pub fn create_table(&mut self, table_name: &str) {
        self.tables.insert(table_name.to_string(), Vec::new());
        print!("Created table: {}", table_name);
    }

    // Scan a table and return all rows
    pub fn scan_table(&self, table: &str) -> Vec<Row> {
        self.tables.get(table).cloned().unwrap_or_default()
    }

    // Scan a table with a filter condition and return matching rows
    pub fn scan_table_with_filter(&self , table : &str,col : &str, val : &str) -> Vec<Row> {
        self.tables
            .get(table)
            .unwrap_or(&vec![])
            .iter()
            .filter(|row| row.get(col).map(|v| v == val).unwrap_or(false))
            .cloned()
            .collect()
    }


    // Insert a new row into a table
    pub fn insert_into_table(&mut self, table_name: &str, row: Row){
        match self.tables.get_mut(table_name) {
            Some(rows) => rows.push(row),
            None => println!("Error: table '{table_name}' does not exist"),
        }
    }
}