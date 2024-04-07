use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tracing::info;

use crate::error::Result;

pub async fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    info!("Server running on http://127.0.0.1:3000");
    
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            let _ = handle_connect(stream).await;
        });
    }

    Ok(())
}

async fn handle_connect(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await?;

    let request_str = String::from_utf8_lossy(&buffer);
    let request = request_str.lines().collect::<Vec<&str>>();

    let Some(first_line) = request.first() else {
        return handle_invalid_request(stream).await;
    };

    info!("{}", first_line);

    let route_parts: Vec<&str> = first_line.split_whitespace().collect();
    let Some(method) = route_parts.get(0) else {
        return handle_invalid_request(stream).await;
    };

    let Some(uri) = route_parts.get(1) else {
        return handle_invalid_request(stream).await;
    };

    let (status_line, filename) = match *uri {
        "/" => ("HTTP/1.1 200 OK", "200 OK"),
        "/sleep" => {
            std::thread::sleep(std::time::Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "200 OK")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404 Not Found"),
    };

    handle_response(stream, status_line, filename).await?;

    Ok(())
}

async fn handle_response(mut stream: TcpStream, status_line: &str, contents: &str) -> Result<()> {
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}

async fn handle_invalid_request(mut stream: TcpStream) -> Result<()> {
    let contents = "400 Bad Request".to_string();
    let response = format!(
        "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}
