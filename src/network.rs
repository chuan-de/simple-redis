use anyhow::Result;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed,Encoder,Decoder};
use crate::{Backend, RespFrame};

#[derive(Debug)]
struct RespFrameCodec {

}

#[derive(Debug)]
struct RedisRequest{
    frame:RespFrame,
    backend:Backend,
}

#[derive(Debug)]
struct RedisResponse {
    frame:RespFrame,
}

pub async fn stream_handler(_stream:TcpStream) -> Result<()> {
    loop {
        todo!()
    }
    Ok(())
}

async fn request_handler(_request:RedisRequest) -> Result<RedisResponse> {
    todo!()
}

impl Encoder <RespFrame> for RespFrameCodec {
    type Error = std::io::Error;
    fn encode(&mut self, item: RespFrame, dst: &mut bytes::BytesMut) -> std::result::Result<(), Self::Error> {
        item.encode(dst);
        Ok(())
    }
}