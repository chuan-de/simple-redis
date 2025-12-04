use anyhow::Result;
use simple_redis::network;
use tokio::net::TcpListener;
use tracing::{info, warn};


#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let addr = "0.0.0.0:6379".parse()?;
    info!("simple-redis-server is listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("new connection from {}", addr);
        tokio::spawn(async move {
            if let Err(e) = network::stream_handler(stream).await {
                warn!("handle stream error: {}:{:?}", addr,e);        
            }
        });
    }
    Ok(())
}
