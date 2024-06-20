use crate::systems::*;
use crate::stress_loop::generic_test;
use crate::rpc_results::RPCResult;
use defichain_rpc::*;
use defichain_rpc_json::account::*;

pub async fn get_account() -> Result<String> {
    generic_test(
        || async {
            let address = String::from("8defichainBurnAddressXXXXXXXdRQkSm");
            let result = connect()
                .await
                .get_account(
                    &address,
                    None,
                    true.into(),
                ).await?;
            Ok(RPCResult::AccountAmount(result))
        }
    ).await
}

pub async fn get_account_history() -> Result<String> {
    generic_test(
        || async {
            let address = String::from("dZcHjYhKtEM88TtZLjp314H2xZjkztXtRc");
            let result= connect()
                .await
                .get_account_history(
                    &address,
                    2433593,
                    19,
                )
                .await?;
            Ok(RPCResult::AccountHistory(result))
        }
    ).await
}

pub async fn list_accounts() -> Result<String> {
    generic_test(
        || async {
            let accounts_pagination = ListAccountsPagination::new(None, None, 2u64.into());
            let result= connect()
                .await
                .list_accounts(
                    accounts_pagination.clone().into(),
                    true.into(),
                    true.into(),
                    true.into(),
                )
                .await?;
            Ok(RPCResult::AccountsResult(result))
        }
    ).await
}

pub async fn list_account_history() -> Result<String> {
    generic_test(
        || async {
            let account_options = AccountHistoryOptions::new(
                None,
                None,
                None,
                None,
                None,
                None,
                100u64.into(),
                None,
                None,
                None,
                None,
            );
            let result= connect()
                .await
                .list_account_history("dZcHjYhKtEM88TtZLjp314H2xZjkztXtRc".to_string().into(), account_options.clone())
                .await?;
            Ok(RPCResult::AccountHistoryList(result))
        }
    ).await
}
