use crate::AppResult;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::{Http, Middleware, Provider, U256};
use ethers::types::Address;
use ethers_signers::{LocalWallet, Signer};

#[derive(Clone, Debug)]
pub struct MyWallet {
    pub address: Address,
    pub wallet: LocalWallet,
    pub provider: Provider<Http>,
}

impl MyWallet {
    pub fn new(raw_secret: String, ethereum_url: String) -> Self {
        let wallet = raw_secret.parse::<LocalWallet>().unwrap();
        let provider = Provider::<Http>::try_from(ethereum_url).unwrap();

        Self {
            address: wallet.address(),
            wallet,
            provider,
        }
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
}

pub fn wei_to_ether(wei_amount: U256) -> f64 {
    let ether_float = wei_amount.to_string().parse::<f64>().unwrap() * (10.0f64).powi(-18);
    ether_float
}

pub fn ether_to_wei(ether_amount: f64) -> U256 {
    let wei_float = ether_amount * (10.0f64).powi(18);
    U256::from(wei_float.round() as u64)
}
