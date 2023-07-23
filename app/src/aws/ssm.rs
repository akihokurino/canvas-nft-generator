use crate::errors::AppError;
use crate::AppResult;

pub async fn may_load_dotenv() -> AppResult<()> {
    let parameter_name = match std::env::var("SSM_DOTENV_PARAMETER_NAME") {
        Ok(v) if !v.is_empty() => v,
        _ => return Ok(()),
    };

    let shared_config = aws_config::load_from_env().await;

    let body = aws_sdk_ssm::Client::new(&shared_config)
        .get_parameter()
        .name(parameter_name)
        .with_decryption(true)
        .send()
        .await?
        .parameter
        .unwrap()
        .value
        .unwrap();

    for (k, v) in dotenv_parser::parse_dotenv(&body)
        .map_err(|_v| AppError::internal("SSMのロードに失敗しました"))?
    {
        std::env::set_var(k, v);
    }

    Ok(())
}
