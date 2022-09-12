extern crate ticker;

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod node_runtime {}

use subxt::{OnlineClient, PolkadotConfig};
use ticker::Ticker;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
            println!("  Dynamic event details: {block_hash:?}:");

            for event in events.iter() {
                let event = event?;
                let is_balance_transfer = event
                    .as_event::<node_runtime::balances::events::Transfer>()?
                    .is_some();
                let pallet = event.pallet_name();
                let variant = event.variant_name();
      
                
                println!("    {pallet}::{variant} (is balance transfer? {is_balance_transfer})");
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
