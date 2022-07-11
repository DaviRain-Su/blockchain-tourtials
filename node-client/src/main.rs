
use codec::Decode;
use subxt::{
    rpc::Rpc,
    storage::{
        StorageClient,
        StorageKeyPrefix,
    },
    ClientBuilder,
    DefaultConfig,
    PolkadotExtrinsicParams,
    StorageEntryKey,
    StorageMapKey,
};

#[subxt::subxt(runtime_metadata_path = "./artifacts/metadata.scale")]
pub mod node_template {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tracing_subscriber::fmt::init();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<node_template::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    // Obtain the storage client wrapper from the API.
    let storage: StorageClient<_> = api.client.storage();

    let mut iter = storage
        .iter::<node_template::ibc::storage::SomeMap>(None)
        .await?;

    println!("\nExample 2. Obtained keys:");
    while let Some((mut key, (value_1, value_2))) = iter.next().await? {
        println!("Key: 0x{}", hex::encode(&key));
        println!("  Value: {:?}", (value_1, value_2));
    }

    let mut iter = storage
        .iter::<node_template::ibc::storage::Something>(None)
        .await?;

    println!("\nExample 2. Obtained keys:");
    while let Some((key, value)) = iter.next().await? {
        println!("Key: 0x{}", hex::encode(&key));
        println!("  Value: {}", value);
    }

    println!("hello, world!");

    Ok(())
}