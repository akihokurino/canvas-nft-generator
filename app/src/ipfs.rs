use crate::errors::AppError;
use crate::AppResult;
use bytes::Bytes;
use reqwest::multipart::Part;
use reqwest::{multipart, Url};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Client {
    base_url: Url,
    key: String,
    secret: String,
}

impl Client {
    pub fn new(base_url: String, key: String, secret: String) -> Self {
        Client {
            base_url: base_url.parse().unwrap(),
            key,
            secret,
        }
    }

    pub async fn upload(&self, byte: Bytes, name: String) -> AppResult<IpfsOutput> {
        let form = multipart::Form::new().part("file", Part::bytes(byte.to_vec()).file_name(name));

        let mut url = self.base_url.to_owned();
        url.set_path("/api/v0/add");

        let resp = reqwest::Client::new()
            .post(url.to_string())
            .multipart(form)
            .basic_auth(&self.key, Some(&self.secret))
            .send()
            .await?
            .error_for_status()?
            .json::<IpfsOutput>()
            .await
            .map_err(AppError::from)?;

        Ok(resp)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpfsOutput {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Hash")]
    pub hash: String,
}
