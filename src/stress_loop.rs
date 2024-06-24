use crate::rpc_results::RPCResult;
use defichain_rpc::Error;
use rayon::prelude::*;
use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::runtime;

const STRESS_LOOPS: u32 = 20;

pub async fn generic_test<F, Fut>(
    func: F,
) -> Result<String, Error>
where
    F: Fn() -> Fut + Sync + Send,
    Fut: Future<Output = Result<RPCResult, Error>>,
{
    let original = func().await?;

    let error_flag = Arc::new(AtomicBool::new(false));
    let result_flag = Arc::new(AtomicBool::new(true));

    (0..STRESS_LOOPS).into_par_iter().for_each(|_| {
        let error_flag = Arc::clone(&error_flag);
        let result_flag = Arc::clone(&result_flag);

        match runtime::Runtime::new() {
            Ok(rt) => {
                rt.block_on(async {
                    match func().await {
                        Ok(result) => {
                            if original != result {
                                result_flag.store(false, Ordering::SeqCst);
                            }
                        }
                        Err(_) => {
                            error_flag.store(true, Ordering::SeqCst);
                        }
                    }
                });
            }
            Err(_) => {
                error_flag.store(false, Ordering::SeqCst);
            }
        }
    });

    if error_flag.load(Ordering::SeqCst) {
        return Err(Error::Custom(
            "Connection failure during parallel loop".into()
        ));
    }

    if result_flag.load(Ordering::SeqCst) {
        Ok("Success: results match".into())
    } else {
        Ok("Failure: results do not match".into())
    }
}
