use defichain_rpc::{bitcoin, Auth, Client, RpcApi};
use std::process;

pub async fn connect() -> Client {
    Client::new(
        "http://localhost:8554",
        Auth::UserPass("user".to_string(), "Qwerty12".to_string()),
    )
    .await
    .unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        process::exit(1);
    })
}

pub async fn rollback(count: u64) -> bitcoin::BlockHash {
    // Get RPC client
    let rpc = connect().await;

    // Get block count
    let block_count = rpc.get_block_count().await.unwrap_or_else(|e| {
        eprintln!("get_block_count error: {e}");
        process::exit(1);
    });

    // Get block hash
    let block_hash = rpc
        .get_block_hash(block_count - count)
        .await
        .unwrap_or_else(|e| {
            eprintln!("get_block_hash error: {e}");
            process::exit(1);
        });

    // Invalidate block
    rpc.invalidate_block(&block_hash).await.unwrap_or_else(|e| {
        eprintln!("invalidate_block error: {e}");
        process::exit(1);
    });

    block_hash
}

pub async fn reconsider_block(block_hash: bitcoin::BlockHash) {
    // Get RPC client
    let rpc = connect().await;

    // Reconsider block
    rpc.reconsider_block(&block_hash).await.unwrap_or_else(|e| {
        eprintln!("reconsider_block error: {e}");
        process::exit(1);
    });
}
