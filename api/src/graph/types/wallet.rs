use crate::graph;
use app::ethereum;

pub struct MyWallet {
    pub my_wallet: ethereum::MyWallet,
}

#[async_graphql::Object]
impl MyWallet {
    async fn address(&self) -> String {
        format!("{:?}", self.my_wallet.address.clone())
    }

    async fn balance(&self) -> graph::Result<f64> {
        let balance = self.my_wallet.get_balance().await?;
        Ok(balance)
    }
}
