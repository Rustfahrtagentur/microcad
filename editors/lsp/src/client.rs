use serde_json::{json, Value};
use std::error::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8080").await?;

    send_request(
        &mut stream,
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "processid": null,
                "rootUri": null,
                "capabilities": {}
            }
        }),
    )
    .await?;

    send_request(
        &mut stream,
        json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "workspace/execute_command",
                "params": {
                    "command": "custom.notification",
                    "arguments": [
                        {
                            "title":"Hello",
                            "message": "Hello from client",
                            "description": "This is a custom notification from client",
                        }
                    ]
                }
        }),
    )
    .await?;

    Ok(())
}

async fn send_request(stream: &mut TcpStream, request: Value) -> Result<String, Box<dyn Error>> {
    let request_str = request.to_string();
    let request_formatted = format!(
        "Content-Length: {}\r\n\r\n{}",
        request_str.len(),
        request_str
    );

    stream.write_all(request_formatted.as_bytes()).await?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[0..n]);

    println!("Received execute command response: {response}");

    Ok(response.to_string())
}
