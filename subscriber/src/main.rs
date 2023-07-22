use app::errors::AppError;
use app::{aws, di, domain, AppResult};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    aws::ssm::may_load_dotenv()
        .await
        .expect("failed to load ssm parameter store");
    lambda_runtime::run(service_fn(bridge)).await?;
    Ok(())
}

async fn bridge(event: LambdaEvent<Value>) -> Result<(), Error> {
    if let Err(err) = exec(event.payload).await {
        println!("subscriber error: {:?}", err);
    }
    Ok(())
}

async fn exec(payload: Value) -> AppResult<()> {
    let data: EventData = serde_json::from_value(payload)?;
    let now = domain::time::now();
    let nft_app = di::NFT_APP.get().await.clone();

    match aws::sns::Task::from_sns(
        data.records
            .first()
            .ok_or_else(|| AppError::bad_request())?
            .sns
            .message
            .clone(),
    )? {
        aws::sns::Task::Mint(payload) => {
            nft_app.mint(payload.work_id, payload.gs_path, now).await?;
            Ok(())
        }
        aws::sns::Task::Transfer(payload) => {
            nft_app
                .transfer(
                    domain::contract::ContractId::from(payload.address),
                    domain::token::TokenId::from(payload.token_id),
                    domain::contract::WalletAddress::from(payload.to_address),
                )
                .await?;
            Ok(())
        }
    }
}

#[derive(Serialize, Deserialize)]
struct EventData {
    #[serde(rename = "Records")]
    pub records: Vec<Record>,
}

#[derive(Serialize, Deserialize)]
struct Record {
    #[serde(rename = "Sns")]
    pub sns: Sns,
}

#[derive(Serialize, Deserialize)]
struct Sns {
    #[serde(rename = "Message")]
    pub message: String,
}
