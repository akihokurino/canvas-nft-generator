pub struct Wallet {
    pub address: String,
    pub balance: f64,
}

#[async_graphql::Object]
impl Wallet {
    async fn address(&self) -> String {
        self.address.clone()
    }

    async fn balance(&self) -> f64 {
        self.balance.clone()
    }
}
