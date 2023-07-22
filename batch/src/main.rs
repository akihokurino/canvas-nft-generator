use app::errors::AppError;
use app::{aws, AppResult};
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
        lambda_runtime::run(service_fn(in_lambda)).await?;
    } else {
        let command = env::var("COMMAND").unwrap_or_default();
        if let Err(err) = handle(&command).await {
            println!("batch error: {:?}", err);
        }
    }

    Ok(())
}

async fn in_lambda(event: LambdaEvent<Value>) -> Result<(), Error> {
    let command = match get_command_from_batch_event(event.payload) {
        Ok(v) => v,
        Err(err) => {
            println!("batch error: {:?}", err);
            return Ok(());
        }
    };

    println!("exec command: {}", command.clone());
    if let Err(err) = handle(&command).await {
        println!("batch error: {:?}", err);
    }

    Ok(())
}

async fn handle(command: &str) -> AppResult<()> {
    if command == "sync-token" {}

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
