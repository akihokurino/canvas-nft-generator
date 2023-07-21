use crate::graph;
use crate::graph::AppContext;
use app::ddb;
use app::{domain, ethereum};
use async_graphql::Object;
use async_graphql::{Context, InputObject};

#[derive(Default)]
pub struct ContractMutation;

#[Object]
impl ContractMutation {
    async fn contract_create(
        &self,
        ctx: &Context<'_>,
        input: ContractCreateInput,
    ) -> graph::Result<graph::types::contract::Contract> {
        ctx.authorized()?;

        let my_wallet = ctx.data::<ethereum::MyWallet>()?;
        let contract_repository = ctx.data::<ddb::contract::Repository>()?;
        let now = domain::time::now();

        let contract = domain::contract::Contract::new(
            domain::contract::ContractId::from(input.address),
            domain::contract::WalletAddress::from(my_wallet.raw_address()),
            input.abi,
            now,
        );

        contract_repository.put(contract.clone()).await?;

        Ok(graph::types::contract::Contract { contract })
    }
}

#[derive(InputObject)]
pub struct ContractCreateInput {
    pub address: String,
    pub abi: String,
}
