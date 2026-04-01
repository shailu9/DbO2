use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use crate::store::{ self, Store };
use crate::parser;
use crate::executor;

pub async fn run(mut store:Store){
    let address ="127.0.0.1:7433";
    let listener = TcpListener::bind(address).await.unwrap();
    println!("OxideateDB listening on {address}");

    loop{
        let (socket,peer) = listener.accept().await.unwrap();
        println!("New client connected: {peer}");

        let (reader, mut writer) = tokio::io::split(socket);
        let mut lines = BufReader::new(reader).lines();

        writer.write_all(b"Welcome to OxidateDB! Type SQL or 'exit'\n").await.unwrap();

        while let Ok(Some(line)) = lines.next_line().await {
            let sql = line.trim().to_string();
            if sql.is_empty() {
                continue;
            }

            if sql.eq_ignore_ascii_case("exit") || sql.eq_ignore_ascii_case("quit") {
                writer.write_all(b"Goodbye!\n").await.unwrap();
                break;
            }

            match parser::parse_sql(&sql) {
                Ok(statements) => {
                    for stmt in statements {
                        let results =executor::execute_statement(stmt, &mut store);
                        writer.write_all(results.as_bytes()).await.unwrap();
                    }
                    writer.write_all(b"OK\n").await.unwrap();
                }
                Err(e) => {
                    let msg = format!("ERROR: {e}\n");
                    writer.write_all(msg.as_bytes()).await.unwrap();
                }
            }
        };
        println!("Connection from {peer} closed");
    };
}