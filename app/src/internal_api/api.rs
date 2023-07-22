pub mod service {
    tonic::include_proto!("service");
}

use crate::internal_api::Client;
use crate::AppResult;
use http::header::{ACCEPT, CONTENT_TYPE};

impl Client {
    pub async fn get_signed_urls(&self, gs_urls: Vec<String>) -> AppResult<Vec<String>> {
        let msg = service::SignedGsUrlsRequest { gs_urls };

        let request = http::Request::builder()
            .method(http::Method::POST)
            .uri(format!(
                "{}service.InternalAPI/SignedGsUrls",
                self.base_url.to_string()
            ))
            .header(CONTENT_TYPE, "application/grpc-web")
            .header(ACCEPT, "application/grpc-web")
            .header("authorization", self.token.to_string())
            .body(hyper::Body::from(self.encode_body(msg)))
            .unwrap();

        let body = self.call(request).await?;
        let reply = self
            .decode_body::<service::SignedGsUrlsResponse>(body)
            .await?;

        Ok(reply.urls)
    }

    pub async fn send_push(&self, text: &str) -> AppResult<()> {
        let msg = service::SendPushRequest {
            text: text.to_owned(),
        };

        let request = http::Request::builder()
            .method(http::Method::POST)
            .uri(format!(
                "{}service.InternalAPI/SendPush",
                self.base_url.to_string()
            ))
            .header(CONTENT_TYPE, "application/grpc-web")
            .header(ACCEPT, "application/grpc-web")
            .header("authorization", self.token.to_string())
            .body(hyper::Body::from(self.encode_body(msg)))
            .unwrap();

        let body = self.call(request).await?;
        let _ = self.decode_body::<service::SendPushResponse>(body).await?;

        Ok(())
    }
}
