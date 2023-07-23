use crate::errors::AppError;
use crate::AppResult;
use aws_sdk_lambda::primitives::Blob;
use aws_sdk_lambda::Client;

#[derive(Clone, Debug)]
pub struct Adapter {
    client: Client,
    lambda_open_sea_arn: String,
}

impl Adapter {
    pub fn new(client: Client, lambda_open_sea_arn: String) -> Self {
        Self {
            client,
            lambda_open_sea_arn,
        }
    }

    pub async fn invoke_lambda_open_sea(
        &self,
        input: invoke_open_sea_sdk::Request,
    ) -> AppResult<invoke_open_sea_sdk::Response> {
        let json = serde_json::to_string(&input)?;
        let resp = self
            .client
            .invoke()
            .function_name(self.lambda_open_sea_arn.clone())
            .payload(Blob::new(json.into_bytes()))
            .send()
            .await?;

        let payload = resp.payload.unwrap();
        let payload = String::from_utf8(payload.into_inner()).ok().unwrap();
        let output: invoke_open_sea_sdk::Response = serde_json::from_str(&payload)?;

        if output.result != 0 {
            return Err(AppError::internal("OpenSea SDKの呼び出しに失敗しました"));
        }

        Ok(output)
    }
}

pub mod invoke_open_sea_sdk {
    use crate::domain::contract::ContractId;
    use crate::domain::token::TokenId;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize)]
    pub struct Request {
        pub method: String,
        #[serde(rename(serialize = "infoRequest"))]
        pub info_request: Option<InfoRequest>,
        #[serde(rename(serialize = "sellRequest"))]
        pub sell_request: Option<SellRequest>,
    }

    #[derive(Debug, Serialize)]
    pub struct InfoRequest {
        #[serde(rename(serialize = "tokenAddress"))]
        pub token_address: String,
        #[serde(rename(serialize = "tokenId"))]
        pub token_id: String,
    }

    #[derive(Debug, Serialize)]
    pub struct SellRequest {
        #[serde(rename(serialize = "tokenAddress"))]
        pub token_address: String,
        #[serde(rename(serialize = "tokenId"))]
        pub token_id: String,
        #[serde(rename(serialize = "ether"))]
        pub ether: f64,
        #[serde(rename(serialize = "quantity"))]
        pub quantity: i32,
        #[serde(rename(serialize = "schema"))]
        pub schema: String,
    }

    impl Request {
        pub fn info(address: &ContractId, token_id: &TokenId) -> Self {
            Self {
                method: "info".to_string(),
                info_request: Some(InfoRequest {
                    token_address: address.clone().to_string(),
                    token_id: token_id.clone().to_string(),
                }),
                sell_request: None,
            }
        }

        pub fn sell(address: &ContractId, token_id: &TokenId, ether: f64) -> Self {
            Self {
                method: "sell".to_string(),
                info_request: None,
                sell_request: Some(SellRequest {
                    token_address: address.clone().to_string(),
                    token_id: token_id.clone().to_string(),
                    ether,
                    quantity: 1,
                    schema: "ERC721".to_string(),
                }),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Response {
        #[serde(rename(deserialize = "result"))]
        pub result: i32,
        #[serde(rename(deserialize = "errorMessage"))]
        pub error_message: Option<String>,
        #[serde(rename(deserialize = "infoResponse"))]
        pub info_response: Option<InfoResponse>,
        #[serde(rename(deserialize = "sellResponse"))]
        pub sell_response: Option<SellResponse>,
    }

    #[derive(Debug, Deserialize)]
    pub struct InfoResponse {
        #[serde(rename(deserialize = "sellPrice"))]
        pub sell_price: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct SellResponse {
        #[serde(rename(deserialize = "sellPrice"))]
        pub sell_price: String,
    }
}
