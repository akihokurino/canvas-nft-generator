use crate::errors::AppError;
use crate::AppResult;
use aws_sdk_sns::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Adapter {
    client: Client,
    topic_arn: String,
}

impl Adapter {
    pub fn new(client: Client, topic_arn: String) -> Self {
        Self { client, topic_arn }
    }

    pub async fn publish(&self, task: Task) -> AppResult<()> {
        self.client
            .publish()
            .topic_arn(&self.topic_arn)
            .message(task.message()?)
            .send()
            .await?;

        Ok(())
    }
}

impl Task {
    pub fn message(&self) -> AppResult<String> {
        serde_json::to_string(self)
            .map_err(|_e| AppError::internal("SNSペイロードの変換に失敗しました"))
    }

    pub fn from_sns(message: String) -> Result<Task, AppError> {
        serde_json::from_str(&message)
            .map_err(|_e| AppError::internal("SNSペイロードの変換に失敗しました"))
    }
}

#[derive(Serialize, Deserialize)]
pub enum Task {
    Mint(MintPayload),
    Sell(SellPayload),
    Transfer(TransferPayload),
}

#[derive(Serialize, Deserialize)]
pub struct MintPayload {
    pub work_id: String,
    pub gs_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct SellPayload {
    pub address: String,
    pub token_id: String,
    pub ether: f64,
}

#[derive(Serialize, Deserialize)]
pub struct TransferPayload {
    pub address: String,
    pub token_id: String,
    pub to_address: String,
}
