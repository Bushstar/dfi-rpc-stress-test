use defichain_rpc_json::account::*;

#[derive(Debug, PartialEq)]
pub enum RPCResult {
    AccountAmount(AccountAmount),
    AccountHistory(AccountHistory),
    AccountHistoryList(Vec<AccountHistory>),
    AccountsResult(Vec<AccountsResult>),
}