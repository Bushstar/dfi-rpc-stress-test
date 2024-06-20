mod account_tests;
mod systems;
mod test_runner;
mod stress_loop;
mod rpc_results;

use crate::test_runner::*;

#[tokio::main]
async fn main() {
    test_runner().await;
}
