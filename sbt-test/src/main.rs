use mpl_core::{
    instructions::{CreateCollectionV2Builder, CreateV1Builder, CreateV2Builder},
    types::{
        ExternalCheckResult, ExternalPluginAdapterInitInfo, HookableLifecycleEvent, Oracle,
        OracleInitInfo, PermanentFreezeDelegate, Plugin, PluginAuthority, PluginAuthorityPair,
        ValidationResultsOffset,
    },
};
use solana_client::nonblocking::rpc_client;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, read_keypair_file},
    signer::Signer,
    transaction::Transaction,
};
use std::{path::Path, str::FromStr};
fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(mint_asset_to_collection())
}

pub fn load_wallet(path: &str) -> Keypair {
    read_keypair_file(Path::new(path)).expect("failed to read keypair")
}

// pub async fn mint_sbt(){
//     let payer = load_wallet("/home/void/.config/solana/turbin3-wallet.json");
//     println!("Keypair is {:?}", payer.pubkey().to_string());

//     let rpc_client = rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());
//     let asset = Keypair::new();
//     let delegate = Pubkey::from_str("11111111111111111111111111111111").unwrap();
// let create_asset_with_permanent_freeze_delegate_plugin_ix = CreateV1Builder::new()
//         .asset(asset.pubkey())
//         .payer(payer.pubkey())
//         .name("My Nft".into())
//         .uri("https://example.com/my-nft.json".into())
//         .plugins(vec![PluginAuthorityPair {
//             plugin: Plugin::PermanentFreezeDelegate(PermanentFreezeDelegate { frozen: true }),
//             authority: Some(PluginAuthority::Address { address: delegate }),
//         }])
//         .instruction();

//     let signers = vec![&asset, &payer];

//     let latest_blockhash = rpc_client.get_latest_blockhash().await.unwrap();

//     let create_sbt = Transaction::new_signed_with_payer(
//         &[create_asset_with_permanent_freeze_delegate_plugin_ix],
//         Some(&payer.pubkey()),
//         &signers,
//         latest_blockhash,
//     );

//     let res = rpc_client
//         .send_and_confirm_transaction(&create_sbt)
//         .await
//         .unwrap();

//     println!("Signature: {:?}", res)
// }

// async fn create_collection() {
//     let payer = load_wallet("/home/void/.config/solana/turbin3-wallet.json");
//     println!("Keypair is {:?}", payer.pubkey().to_string());

//     let rpc_client = rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());
//     let collection = Keypair::new();
//     println!("{:?}", collection.pubkey());
//     let onchain_oracle_plugin =
//         Pubkey::from_str("GxaWxaQVeaNeFHehFQEDeKR65MnT6Nup81AGwh2EEnuq").unwrap();
//     let create_collection_with_oracle_plugin_ix = CreateCollectionV2Builder::new()
//         .collection(collection.pubkey())
//         .payer(payer.pubkey())
//         .name("My Collection".into())
//         .uri("https://example.com/my-collection.json".into())
//         .external_plugin_adapters(vec![ExternalPluginAdapterInitInfo::Oracle(
//             OracleInitInfo {
//                 base_address: onchain_oracle_plugin,
//                 init_plugin_authority: None,
//                 lifecycle_checks: vec![(
//                     HookableLifecycleEvent::Transfer,
//                     ExternalCheckResult { flags: 4 },
//                 )],
//                 base_address_config: None,
//                 results_offset: Some(ValidationResultsOffset::Anchor),
//             },
//         )])
//         .instruction();
//     let signers = vec![&collection, &payer];

//     let last_blockhash = rpc_client.get_latest_blockhash().await.unwrap();

//     let create_collection_with_oracle_plugin_tx = Transaction::new_signed_with_payer(
//         &[create_collection_with_oracle_plugin_ix],
//         Some(&payer.pubkey()),
//         &signers,
//         last_blockhash,
//     );

//     let res = rpc_client
//         .send_and_confirm_transaction(&create_collection_with_oracle_plugin_tx)
//         .await
//         .unwrap();

//     println!("Signature: {:?}", res)
// }

async fn mint_asset_to_collection() {
    let rpc_client = rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());
    let asset = Keypair::new(); // new NFT/asset
    let payer = load_wallet("/home/void/.config/solana/turbin3-wallet.json");

    let collection_mint = Pubkey::from_str("EGQaJEbz6pR3QT1ZnF2cSaGinu8wQykknPyrNUg9WnV3").unwrap();

    // Create a basic NFT pointing to the collection
    let ix = CreateV2Builder::new()
        .asset(asset.pubkey())
        .payer(payer.pubkey())
        .name("My NFT #1".into())
        .uri("https://example.com/my-nft.json".into())
        .collection(Some(collection_mint)) // this links it to your collection
        .instruction();

    let signers = vec![&asset, &payer];
    let recent_blockhash = rpc_client.get_latest_blockhash().await.unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &signers,
        recent_blockhash,
    );

    let sig = rpc_client.send_and_confirm_transaction(&tx).await.unwrap();
    println!("Minted NFT with signature: {}", sig);
}
