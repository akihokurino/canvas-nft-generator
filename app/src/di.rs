use crate::sync::LoadOnce;
use crate::{application, aws, ddb, ethereum, internal_api, ipfs};
use once_cell::sync::Lazy;

pub type LazyAsync<T> = Lazy<LoadOnce<T>>;

#[macro_export]
macro_rules! lazy_async {
    ( $x: expr ) => {
        $crate::sync::Lazy::new(|| $crate::sync::LoadOnce::lazy($x))
    };
}

pub fn must_env(k: &str) -> String {
    std::env::var(k).expect(format!("env {} missing", k).as_str())
}

pub static GRPC_SERVER_BASE_URL: Lazy<String> = Lazy::new(|| must_env("GRPC_SERVER_BASE_URL"));
pub static INTERNAL_TOKEN: Lazy<String> = Lazy::new(|| must_env("INTERNAL_TOKEN"));
pub static OPEN_SEA_BASE_URL: Lazy<String> = Lazy::new(|| must_env("OPEN_SEA_BASE_URL"));
pub static ETHEREUM_URL: Lazy<String> = Lazy::new(|| must_env("ETHEREUM_URL"));
pub static LAMBDA_OPEN_SEA_ARN: Lazy<String> = Lazy::new(|| must_env("LAMBDA_OPEN_SEA_ARN"));
pub static IPFS_URL: Lazy<String> = Lazy::new(|| must_env("IPFS_URL"));
pub static IPFS_KEY: Lazy<String> = Lazy::new(|| must_env("IPFS_KEY"));
pub static IPFS_SECRET: Lazy<String> = Lazy::new(|| must_env("IPFS_SECRET"));
pub static IPFS_GATEWAY: Lazy<String> = Lazy::new(|| must_env("IPFS_GATEWAY"));
pub static WALLET_ADDRESS: Lazy<String> = Lazy::new(|| must_env("WALLET_ADDRESS"));
pub static WALLET_SECRET: Lazy<String> = Lazy::new(|| must_env("WALLET_SECRET"));
pub static SNS_TOPIC_ARN: Lazy<String> = Lazy::new(|| must_env("SNS_TOPIC_ARN"));

pub static MY_WALLET: LazyAsync<ethereum::MyWallet> = lazy_async!(async {
    ethereum::MyWallet::new(
        WALLET_SECRET.clone(),
        ETHEREUM_URL.clone(),
        INTERNAL_TOKEN.clone(),
    )
});
pub static INTERNAL_API_CLIENT: LazyAsync<internal_api::Client> = lazy_async!(async {
    internal_api::Client::new(GRPC_SERVER_BASE_URL.clone(), INTERNAL_TOKEN.clone())
});
pub static IPFS_CLIENT: LazyAsync<ipfs::Client> = lazy_async!(async {
    ipfs::Client::new(IPFS_URL.clone(), IPFS_KEY.clone(), IPFS_SECRET.clone())
});
pub static CANVAS: LazyAsync<ethereum::canvas::Canvas> = lazy_async!(async {
    ethereum::canvas::Canvas::new(WALLET_SECRET.clone(), ETHEREUM_URL.clone())
});

static AWS_CONFIG: LazyAsync<aws_config::SdkConfig> = lazy_async!(aws_config::load_from_env());
static DDB_CLIENT: LazyAsync<aws_sdk_dynamodb::Client> =
    lazy_async!(async { aws_sdk_dynamodb::Client::new(AWS_CONFIG.get().await) });
static SNS_CLIENT: LazyAsync<aws_sdk_sns::Client> =
    lazy_async!(async { aws_sdk_sns::Client::new(AWS_CONFIG.get().await) });

pub static SNS_ADAPTER: LazyAsync<aws::sns::Adapter> = lazy_async!(async {
    aws::sns::Adapter::new(SNS_CLIENT.get().await.clone(), SNS_TOPIC_ARN.clone())
});

pub static CONTRACT_REPOSITORY: LazyAsync<ddb::contract::Repository> =
    lazy_async!(async { ddb::contract::Repository::new(DDB_CLIENT.get().await.clone()) });
pub static TOKEN_REPOSITORY: LazyAsync<ddb::token::Repository> =
    lazy_async!(async { ddb::token::Repository::new(DDB_CLIENT.get().await.clone()) });

pub static NFT_APP: LazyAsync<application::nft::NftApp> = lazy_async!(async {
    application::nft::NftApp::new(
        MY_WALLET.get().await.clone(),
        INTERNAL_API_CLIENT.get().await.clone(),
        IPFS_CLIENT.get().await.clone(),
        CANVAS.get().await.clone(),
        CONTRACT_REPOSITORY.get().await.clone(),
        TOKEN_REPOSITORY.get().await.clone(),
    )
});
