pub mod canvas;

use crate::domain::contract::ContractId;
use crate::domain::token::TokenId;
use crate::errors::AppError;
use crate::AppResult;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::types::Address;
use ethers_signers::{LocalWallet, Signer};
use std::str::FromStr;

pub const GAS_LIMIT: i64 = 8000000;
pub const GAS_PRICE: i64 = 25000000000; // 40000000000

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

impl From<U256> for TokenId {
    fn from(value: U256) -> Self {
        Self::new(value.to_string())
    }
}

impl TryInto<U256> for TokenId {
    type Error = AppError;

    fn try_into(self) -> Result<U256, Self::Error> {
        U256::from_dec_str(&self.to_string()).map_err(|_e| AppError::internal())
    }
}

impl From<Address> for ContractId {
    fn from(value: Address) -> Self {
        Self::new(format!("{:?}", value))
    }
}

impl TryInto<Address> for TokenId {
    type Error = AppError;

    fn try_into(self) -> Result<Address, Self::Error> {
        self.to_string()
            .parse::<Address>()
            .map_err(|_e| AppError::internal())
    }
}
