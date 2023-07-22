use crate::graph;
use crate::graph::AppContext;
use app::{application, aws, ddb};
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

        let sns_adapter = ctx.data::<aws::sns::Adapter>()?;
        let nft_app = ctx.data::<application::nft::NftApp>()?;
        let now = domain::time::now();

        if input.is_async {
            sns_adapter
                .publish(aws::sns::Task::Mint(aws::sns::MintPayload {
                    work_id: input.work_id,
                    gs_path: input.gs_path,
                }))
                .await?;
        } else {
            nft_app.mint(input.work_id, input.gs_path, now).await?;
        }

        Ok(true)
    }

    async fn transfer(&self, ctx: &Context<'_>, input: TransferInput) -> graph::Result<bool> {
        ctx.authorized()?;

        let sns_adapter = ctx.data::<aws::sns::Adapter>()?;
        let nft_app = ctx.data::<application::nft::NftApp>()?;

        if input.is_async {
            sns_adapter
                .publish(aws::sns::Task::Transfer(aws::sns::TransferPayload {
                    address: input.address,
                    token_id: input.token_id,
                    to_address: input.to_address,
                }))
                .await?;
        } else {
            nft_app
                .transfer(
                    domain::contract::ContractId::from(input.address),
                    domain::token::TokenId::from(input.token_id),
                    domain::contract::WalletAddress::from(input.to_address),
                )
                .await?;
        }

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

#[derive(InputObject)]
pub struct TransferInput {
    pub address: String,
    pub token_id: String,
    pub to_address: String,
    pub is_async: bool,
}
