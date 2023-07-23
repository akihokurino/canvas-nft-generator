use app::errors::AppError;
use app::{aws, di, AppResult};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;
use std::env;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Error> {
    aws::ssm::may_load_dotenv()
        .await
        .expect("failed to load ssm parameter store");

    let with_lambda: bool = env::var("WITH_LAMBDA")
        .map(|v| bool::from_str(&v).expect("failed to parse WITH_LAMBDA"))
        .unwrap_or(false);

    if with_lambda {
        lambda_runtime::run(service_fn(bridge)).await?;
    } else {
        let command = env::var("COMMAND").unwrap_or_default();
        if let Err(err) = handle(serde_json::from_str(&command).unwrap()).await {
            println!("batch error: {:?}", err);
        }
    }

    Ok(())
}

async fn bridge(event: LambdaEvent<Value>) -> Result<(), Error> {
    if let Err(err) = handle(event.payload).await {
        println!("batch error: {:?}", err);
    }
    Ok(())
}

async fn handle(payload: Value) -> AppResult<()> {
    let command = get_command_from_batch_event(payload)?;
    let nft_app = di::NFT_APP.get().await.clone();

    println!("exec command: {}", command);
    if command == "sync-token" {
        nft_app.sync().await?;
    }

    Ok(())
}

fn get_command_from_batch_event(event: Value) -> AppResult<String> {
    match event {
        Value::Object(data) => {
            let command = data.get("command");
            if command.is_none() {
                return Err(AppError::bad_request());
            }
            match command.unwrap() {
                Value::String(val) => Ok(val.to_string()),
                _ => Err(AppError::bad_request()),
            }
        }
        _ => Err(AppError::bad_request()),
    }
}
