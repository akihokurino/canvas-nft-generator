pub struct Contract {
    pub address: String,
}

#[async_graphql::Object]
impl Contract {
    async fn address(&self) -> String {
        self.address.clone()
    }
}
