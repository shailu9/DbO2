use std::io::Write;
use std::{ io };
mod store; // Import the store module
mod parser;
mod executor;

fn main() {
    let mut store = store::Store::load(); // Create a new store instance
    println!("OxidateDB v0.1.0 — type SQL or 'exit'");
    // Loop until we get exit command
    loop {
        print!("OxidateDB> ");
        // Flush to ensure the prompt is printed before we read input
        io::stdout().flush().unwrap();

        // Read a line of input from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let sql = input.trim();
        if sql.eq_ignore_ascii_case("exit") || sql.eq_ignore_ascii_case("quit") {
            println!("Bye!");
            break;
        }

        // Here we will parse the SQL and execute it against our store
        // Matche expression helps us handle both the success and error cases of parsing the SQL
        match parser::parse_sql(sql) {
            // If parsing is successful, we get a vector of statements that we can iterate over and execute
            Ok(statements) => {
                for stmt in statements {
                    executor::execute_statement(stmt, &mut store);
                }
            }
            // If there was an error parsing the SQL, we print the error and exit with a non-zero status code
            Err(e) => {
                eprintln!("Parse error: {e}");
                std::process::exit(1);
            }
        }
    }
}