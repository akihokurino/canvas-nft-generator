pub mod api;

use crate::errors::AppError;
use crate::AppResult;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use http::Request;
use hyper::Body;
use reqwest::Url;

const GRPC_HEADER_SIZE: usize = 5;

#[derive(Clone, Debug)]
pub struct Client {
    base_url: Url,
    token: String,
}

impl Client {
    pub fn new(base_url: String, token: String) -> Self {
        Client {
            base_url: base_url.parse().unwrap(),
            token,
        }
    }

    pub async fn call(&self, req: Request<Body>) -> AppResult<Body> {
        let https = hyper_rustls::HttpsConnector::with_native_roots();
        let client = hyper::Client::builder().build::<_, Body>(https);
        let response = client.request(req).await.unwrap();
        Ok(response.into_body())
    }

    pub fn encode_body<T>(&self, msg: T) -> Bytes
    where
        T: prost::Message,
    {
        let mut buf = BytesMut::with_capacity(1024);

        buf.reserve(GRPC_HEADER_SIZE);
        unsafe {
            buf.advance_mut(GRPC_HEADER_SIZE);
        }

        msg.encode(&mut buf).unwrap();

        let len = buf.len() - GRPC_HEADER_SIZE;
        {
            let mut buf = &mut buf[..GRPC_HEADER_SIZE];
            buf.put_u8(0);
            buf.put_u32(len as u32);
        }

        buf.split_to(len + GRPC_HEADER_SIZE).freeze()
    }

    pub async fn decode_body<T>(&self, body: Body) -> AppResult<T>
    where
        T: Default + prost::Message,
    {
        let mut body = hyper::body::to_bytes(body).await.unwrap();
        if body.is_empty() {
            return Err(AppError::not_found());
        }

        body.advance(1);

        let len = body.get_u32();
        let msg = T::decode(&mut body.split_to(len as usize)).unwrap();

        Ok(msg)
    }
}
