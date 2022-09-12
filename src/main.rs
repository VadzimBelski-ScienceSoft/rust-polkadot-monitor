#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod node_runtime {}

use subxt::{OnlineClient, PolkadotConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();

    if let Err(_e) = node_runtime::validate_codegen(&api) {
        println!(r#"Generated code is not up to date with node we're connected to"#);
    }

    let block_number = 21646u32;

    let block_hash = api.rpc().block_hash(Some(block_number.into())).await?;

    if block_hash != None {
        println!(
            r#"Block hash from number: {} block_hash: {:?}"#,
            block_number, block_hash
        );

        // if let Ok(Some(fullblock)) = api.rpc().block(block_hash.into()).await
        //     {
        //     // block.
        //     for extr in fullblock.block.extrinsics {

        //         println!("Hello Extrinsics {:?}",extr);
        //     
        // }

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
        } else {
            println!("  - No balance transfer event found in this block");
        }

        if let Some(hash) = block_hash {
            println!("Block hash for block number {block_number}: {hash}");
        } else {
            println!("Block number {block_number} not found.");
        }
    }

    Ok(())
}
