use crate::graph;
use crate::graph::AppContext;
use app::{application, ddb};
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

    async fn mint(&self, ctx: &Context<'_>, input: MintInput) -> graph::Result<bool> {
        ctx.authorized()?;

        let nft_app = ctx.data::<application::nft::NftApp>()?;
        let now = domain::time::now();

        nft_app.mint(input.work_id, input.gs_path, now).await?;

        Ok(true)
    }
}

#[derive(InputObject)]
pub struct ContractCreateInput {
    pub address: String,
    pub abi: String,
}

#[derive(InputObject)]
pub struct MintInput {
    pub work_id: String,
    pub gs_path: String,
    pub is_async: bool,
}
