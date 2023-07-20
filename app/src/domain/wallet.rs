use derive_more::{From, Into};

#[derive(From, Into, Clone, Debug)]
pub struct WalletAddress(pub String);

#[derive(Clone, Debug)]
pub struct Wallet {
    pub address: WalletAddress,
    pub secret: String,
}
