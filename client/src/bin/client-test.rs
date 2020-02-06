#![allow(dead_code)]

use client::*;

async fn crash(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    let size = 1024*1024*1024*4;
    let resp = client.crash(CrashRequest { size })
        .await
        .map_err(|e| Error {
            kind: ErrorKind::Internal,
            message: format!("{:?}", e),

        })?;
    println!("{:?} MiB", resp.payload.len());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut client = Client::new("localhost", 50056).await?;
    crash(&mut client).await?;
    Ok(())
}
