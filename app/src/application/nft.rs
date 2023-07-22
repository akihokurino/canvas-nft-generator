use crate::domain::contract::Schema;
use crate::domain::time::LocalDateTime;
use crate::domain::token::TokenId;
use crate::open_sea::metadata::Metadata;
use crate::{ddb, domain, ethereum, internal_api, ipfs, AppResult};
use bytes::Bytes;

#[derive(Clone, Debug)]
pub struct NftApp {
    pub my_wallet: ethereum::MyWallet,
    pub internal_api_client: internal_api::Client,
    pub ipfs_client: ipfs::Client,
    pub canvas: ethereum::canvas::Canvas,
    pub contract_repository: ddb::contract::Repository,
    pub token_repository: ddb::token::Repository,
}

impl NftApp {
    pub fn new(
        my_wallet: ethereum::MyWallet,
        internal_api_client: internal_api::Client,
        ipfs_client: ipfs::Client,
        canvas: ethereum::canvas::Canvas,
        contract_repository: ddb::contract::Repository,
        token_repository: ddb::token::Repository,
    ) -> Self {
        NftApp {
            my_wallet,
            internal_api_client,
            ipfs_client,
            canvas,
            contract_repository,
            token_repository,
        }
    }

    pub async fn mint(
        &self,
        work_id: String,
        gs_path: String,
        now: LocalDateTime,
    ) -> AppResult<bool> {
        let contract = self
            .contract_repository
            .get_latest_by_wallet_address_and_schema(
                &self.my_wallet.address.into(),
                &Schema::ERC721,
            )
            .await?;

        let urls = self
            .internal_api_client
            .get_signed_urls(vec![gs_path.to_owned()])
            .await?;
        let url = urls.first().unwrap();
        let bytes = reqwest::get(url).await?.bytes().await?;

        let content_hash = self.ipfs_client.upload(bytes, work_id.clone()).await?;
        println!(
            "image url: {:?}",
            format!("ipfs://{}", content_hash.hash.clone())
        );

        let metadata = Metadata::new(
            &work_id,
            "canvas nft",
            &format!("ipfs://{}", content_hash.hash.clone()),
        );
        let content_hash = self
            .ipfs_client
            .upload(
                Bytes::from(serde_json::to_string(&metadata)?),
                work_id.clone(),
            )
            .await?;
        println!(
            "metadata url: {:?}",
            format!("ipfs://{}", content_hash.hash.clone())
        );

        self.canvas
            .mint(&contract, content_hash.hash.clone())
            .await?;

        let token_id = self
            .canvas
            .token_id_of(&contract, content_hash.hash.clone())
            .await?;

        let token = domain::token::Token::new(
            contract.address,
            TokenId::from(token_id),
            work_id,
            metadata.image,
            metadata.name,
            metadata.description,
            contract.wallet_address,
            now,
        );

        self.token_repository.put(token).await?;

        Ok(true)
    }
}
