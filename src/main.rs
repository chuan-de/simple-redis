use anyhow::Result;
use simple_redis::{Backend, network};
use tokio::net::TcpListener;
use tracing::{info, warn};


#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let addr = "0.0.0.0:6379";
    info!("Simple-Redis-Server is listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    let backend = Backend::new();
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("new connection from {}", addr);
        let backend = backend.clone();  
        tokio::spawn(async move {
             match network::stream_handler(stream,backend).await {
                Ok(()) => info!("connection from {} closed", addr),
                Err(e) => warn!("handle stream error: {}:{:?}", addr,e),     
            }
        });
    }
    Ok(())
}
