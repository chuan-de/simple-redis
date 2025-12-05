use anyhow::Result;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed,Encoder,Decoder};
use crate::{Backend, RespEncode, RespFrame};

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
    type Error = anyhow::Error;
    fn encode(&mut self, item: RespFrame, dst: &mut bytes::BytesMut) -> std::result::Result<(), Self::Error> {
       let encoded = item.encode();
       dst.extend_from_slice(&encoded);
       Ok(())
    }
}

impl Decoder for RespFrameCodec{
    type Error = anyhow::Error;
    type Item = RespFrame;
    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<RespFrame>,Self::Error> {
        let frame = RespFrame::decode(src)?;
        Ok(Some(frame))
    }
}