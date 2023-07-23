use crate::domain::contract::WalletAddress;
use crate::ethereum::{GAS_LIMIT, GAS_PRICE};
use crate::{domain, AppResult};
use ethers::abi::Abi;
use ethers::prelude::*;
use ethers_signers::LocalWallet;
use serde_json::from_str;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Canvas {
    pub wallet: LocalWallet,
    pub provider: Provider<Http>,
}

impl Canvas {
    pub fn new(raw_secret: String, ethereum_url: String) -> Self {
        let wallet = raw_secret.parse::<LocalWallet>().unwrap();
        let provider = Provider::<Http>::try_from(ethereum_url).unwrap();

        Self { wallet, provider }
    }

    fn query_contract(
        &self,
        contract: &domain::contract::Contract,
    ) -> AppResult<Contract<Provider<Http>>> {
        let provider = Arc::new(self.provider.clone());
        let abi: Result<Abi, _> = from_str(&contract.abi);
        Ok(Contract::new(
            contract.address.to_string().parse::<Address>()?,
            abi?,
            provider,
        ))
    }

    async fn transaction_contract(
        &self,
        contract: &domain::contract::Contract,
    ) -> AppResult<Contract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>>
    {
        let client =
            SignerMiddleware::new_with_provider_chain(self.provider.clone(), self.wallet.clone())
                .await
                .unwrap();
        let client = Arc::new(client);
        let abi: Result<Abi, _> = from_str(&contract.abi);
        Ok(Contract::<
            SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
        >::new(
            contract.address.to_string().parse::<Address>()?,
            abi?,
            client.clone(),
        ))
    }

    pub async fn name(&self, contract: &domain::contract::Contract) -> AppResult<String> {
        let res = self
            .query_contract(contract)?
            .method::<_, String>("name", ())?
            .call()
            .await?;
        Ok(res)
    }

    pub async fn owner_of(
        &self,
        contract: &domain::contract::Contract,
        token_id: U256,
    ) -> AppResult<Address> {
        let res = self
            .query_contract(contract)?
            .method::<_, Address>("ownerOf", token_id)?
            .call()
            .await?;
        Ok(res)
    }

    pub async fn token_id_of(
        &self,
        contract: &domain::contract::Contract,
        ipfs_hash: String,
    ) -> AppResult<U256> {
        let res = self
            .query_contract(contract)?
            .method::<_, U256>("tokenIdOf", ipfs_hash)?
            .call()
            .await?;
        Ok(res)
    }

    pub async fn mint(
        &self,
        contract: &domain::contract::Contract,
        ipfs_hash: String,
    ) -> AppResult<()> {
        let to: Address = contract.wallet_address.to_string().parse::<Address>()?;

        let call = self
            .transaction_contract(contract)
            .await?
            .method::<_, H256>("mint", (to, ipfs_hash))?
            .gas(GAS_LIMIT)
            .gas_price(GAS_PRICE);
        let tx = call.send().await?;
        let receipt = tx.await?;

        println!("{:?}", receipt);

        Ok(())
    }

    pub async fn transfer(
        &self,
        contract: &domain::contract::Contract,
        token: &domain::token::Token,
        to: WalletAddress,
    ) -> AppResult<()> {
        let token_id: U256 = token.token_id.clone().try_into()?;
        let from: Address = contract.wallet_address.to_string().parse::<Address>()?;
        let to: Address = to.to_string().parse::<Address>()?;

        let call = self
            .transaction_contract(contract)
            .await?
            .method::<_, H256>("safeTransferFrom", (from, to, token_id))?
            .gas(GAS_LIMIT)
            .gas_price(GAS_PRICE);
        let tx = call.send().await?;
        let receipt = tx.await?;

        println!("{:?}", receipt);

        Ok(())
    }
}
