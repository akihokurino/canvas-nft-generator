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
    if let Err(err) = handle(event.payload).await {
        println!("subscriber error: {:?}", err);
    }
    Ok(())
}

async fn handle(payload: Value) -> AppResult<()> {
    let nft_app = di::NFT_APP.get().await.clone();
    let internal_api_client = di::INTERNAL_API_CLIENT.get().await.clone();
    let ses_adapter = di::SES_ADAPTER.get().await.clone();

    let data: EventData = serde_json::from_value(payload)?;
    let now = domain::time::now();

    match aws::sns::Task::from_sns(
        data.records
            .first()
            .ok_or_else(|| AppError::bad_request("JSONが不正です"))?
            .sns
            .message
            .clone(),
    )? {
        aws::sns::Task::Mint(payload) => {
            nft_app.mint(payload.work_id, payload.gs_path, now).await?;

            internal_api_client
                .send_push("NFTの発行が完了しました")
                .await?;
            ses_adapter
                .send(
                    "aki030402@gmail.com",
                    "Canvasからのお知らせ",
                    "NFTの発行が完了しました",
                )
                .await?;

            Ok(())
        }
        aws::sns::Task::Sell(payload) => {
            nft_app
                .sell(
                    domain::contract::ContractId::from(payload.address.clone()),
                    domain::token::TokenId::from(payload.token_id.clone()),
                    payload.ether,
                )
                .await?;

            internal_api_client
                .send_push("売り注文が完了しました")
                .await?;
            ses_adapter
                .send(
                    "aki030402@gmail.com",
                    "Canvasからのお知らせ",
                    format!(
                        "売り注文が完了しました。 {}#{}",
                        payload.address, payload.token_id
                    )
                    .as_str(),
                )
                .await?;

            Ok(())
        }
        aws::sns::Task::Transfer(payload) => {
            nft_app
                .transfer(
                    domain::contract::ContractId::from(payload.address.clone()),
                    domain::token::TokenId::from(payload.token_id.clone()),
                    domain::contract::WalletAddress::from(payload.to_address),
                )
                .await?;

            internal_api_client
                .send_push("NFTの送付が完了しました")
                .await?;
            ses_adapter
                .send(
                    "aki030402@gmail.com",
                    "Canvasからのお知らせ",
                    format!(
                        "NFTの送付が完了しました。 {}#{}",
                        payload.address, payload.token_id
                    )
                    .as_str(),
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
