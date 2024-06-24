use crate::account_tests::*;
use crate::systems::{reconsider_block, rollback};

pub async fn test_runner() {
    // Rollback chain
    let block_hash = rollback(10).await;

    // Stress test getaccount
    match get_account().await {
        Ok(res) => println!("get_accounts: {res}"),
        Err(e) => println!("get_accounts: {e}"),
    }

    // Stress test getaccounthistory
    match get_account_history().await {
        Ok(res) => println!("get_account_history: {res}"),
        Err(e) => println!("get_account_history: {e}"),
    }

    // Stress test listaccounts
    match list_accounts().await {
        Ok(res) => println!("list_accounts: {res}"),
        Err(e) => println!("list_accounts: {e}"),
    }

    // Stress test listaccounthistory
    match list_account_history().await {
        Ok(res) => println!("list_account_history: {res}"),
        Err(e) => println!("list_account_history: {e}"),
    }

    // Restore chain
    reconsider_block(block_hash).await;
}
