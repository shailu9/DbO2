use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: oxidated <sql>");
        std::process::exit(1);
    }

    let sql = &args[1];
    println!("Executing SQL: {}", sql);
}
