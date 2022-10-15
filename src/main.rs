extern crate ticker;

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod node_runtime {}

use subxt::{OnlineClient, PolkadotConfig};
use ticker::Ticker;
use std::time::Duration;


use bip39::{Language, Mnemonic, MnemonicType};

use sp_core::{sr25519, ed25519, Pair};

/*
  You need to use Archive node so you can replay the history and recover in case long downtime.

  An archive node keeps all the past blocks. An archive node makes it convenient to query the past state of the chain at any point in time. 
  Finding out what an account's balance at a certain block was, or which extrinsics resulted in a certain state change are fast operations when using an archive node. 
  However, an archive node takes up a lot of disk space - around Kusama's 12 millionth block this was around 660 GB.
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    get_address();


    tracing_subscriber::fmt::init();

    let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();

    if let Err(_e) = node_runtime::validate_codegen(&api) {
        println!(r#"Generated code is not up to date with node we're connected to"#);
    }

    println!("Everything working good!");

    let ticker = Ticker::new(0.., Duration::from_secs(1));

    let latest_block_hash = api.rpc().block_hash(None).await.unwrap();
    let latest_block = api.rpc().block(latest_block_hash).await.unwrap();

    let mut block_number = latest_block.as_ref().unwrap().block.header.number;

    for _ in ticker {
        

        let block_hash = api.rpc().block_hash(Some(block_number.into())).await?;

        if block_hash != None {

            println!(
                r#"Block hash from number: {} block_hash: {:?}"#,
                block_number, block_hash
            );
    
            let events = api.events().at(block_hash.into()).await?;
    
            /*
            We can dynamically decode events:
            */
            println!("Dynamic event details: {block_hash:?}:");

            for event in events.iter() {
                let event = event?;
                let is_balance_transfer = event
                    .as_event::<node_runtime::balances::events::Transfer>()?
                    .is_some();
                let pallet = event.pallet_name();
                let variant = event.variant_name();
                
                println!("{pallet}::{variant} (is balance transfer? {is_balance_transfer})");
            }
    
            // Or we can find the first transfer event, ignoring any others:
            let transfer_event = events.find_first::<node_runtime::balances::events::Transfer>()?;
    
            if let Some(ev) = transfer_event {
                println!("  - Balance transfer success: value: {:?}", ev.amount);

                // Lets do Scan !

            } else {
                println!("  - No balance transfer event found in this block");
            }
    
            // after scan lets increment block
            block_number += 1;

        } else {
            println!("No more blocks");
        }
    }

    Ok(())
}

/*

If you would like to create and manage several accounts on the network using the same seed, you can use derivation paths. 
We can think of the derived accounts as child accounts of the root account created using the original mnemonic seed phrase. 
Many Polkadot key generation tools support hard and soft derivation. 
For instance, if you intend to create an account to be used on the Polkadot chain, you can derive a hard key child account using // after the mnemonic phrase.

'caution juice atom organ advance problem want pledge someone senior holiday very//0'

and a soft key child account using / after the mnemonic phrase

'caution juice atom organ advance problem want pledge someone senior holiday very/0'

If you would like to create another account for using the Polkadot chain using the same seed, you can change the number at the end of the string above. 
For example, /1, /2, and /3 will create different derived accounts.

There is an additional type of derivation called password derivation. On Polkadot you can derive a password key account using /// after the mnemonic phrase

'caution juice atom organ advance problem want pledge someone senior holiday very///0'

In this type of derivation, if the mnemonic phrase would leak, accounts cannot be derived without the initial password. 
In fact, for soft- and hard-derived accounts, if someone knows the mnemonic phrase and the derivation path, they will have access to your account.

*/

fn get_address() {

    // https://github.com/paritytech/substrate/blob/0ba251c9388452c879bfcca425ada66f1f9bc802/client/cli/src/commands/generate.rs

    let words = MnemonicType::Words12;
    let mnemonic = Mnemonic::new(words, Language::English);

    let derevative_address = format!("{}/{}", mnemonic.to_string(), "1");

    let pair1 = sr25519::Pair::from_string(&derevative_address, None).unwrap();

    println!("Public key 1 sr25519: {}", pair1.public());

    let derevative_address = format!("{}/{}", mnemonic.to_string(), "2");

    let pair2 = sr25519::Pair::from_string(&derevative_address, None).unwrap();

    println!("Public key 2 sr25519: {}", pair2.public());


    let pair3 = ed25519::Pair::from_string(&mnemonic.to_string(), None).unwrap();

    println!("Public key ed25519: {}", pair3.public());

}