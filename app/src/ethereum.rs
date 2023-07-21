use crate::errors::AppError;
use crate::AppResult;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::types::Address;
use ethers_signers::{LocalWallet, Signer};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct MyWallet {
    pub address: Address,
    pub wallet: LocalWallet,
    pub provider: Provider<Http>,
    pub internal_token: String,
}

impl MyWallet {
    pub fn new(raw_secret: String, ethereum_url: String, internal_token: String) -> Self {
        let wallet = raw_secret.parse::<LocalWallet>().unwrap();
        let provider = Provider::<Http>::try_from(ethereum_url).unwrap();

        Self {
            address: wallet.address(),
            wallet,
            provider,
            internal_token,
        }
    }

    pub fn raw_address(&self) -> String {
        format!("{:?}", self.address)
    }

    pub async fn get_balance(&self) -> AppResult<f64> {
        let client = SignerMiddleware::new_with_provider_chain(
            self.provider.to_owned(),
            self.wallet.to_owned(),
        )
        .await
        .unwrap();

        let balance = client
            .get_balance(self.wallet.address(), None)
            .await
            .unwrap();
        Ok(wei_to_ether(balance))
    }

    pub fn verify(&self, signature: String) -> AppResult<()> {
        let sig = Signature::from_str(&signature).map_err(|_e| AppError::un_authorized())?;
        match sig.verify(self.internal_token.clone(), self.wallet.address().clone()) {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError::un_authorized()),
        }
    }
}

pub fn wei_to_ether(wei_amount: U256) -> f64 {
    let ether_float = wei_amount.to_string().parse::<f64>().unwrap() * (10.0f64).powi(-18);
    ether_float
}

pub fn ether_to_wei(ether_amount: f64) -> U256 {
    let wei_float = ether_amount * (10.0f64).powi(18);
    U256::from(wei_float.round() as u64)
}
