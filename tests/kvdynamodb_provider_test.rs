use kvdynamodb::*;
use std::time::Duration;
use wasmbus_rpc::{
    error::{RpcError, RpcResult},
    provider::prelude::Context,
};
use wasmcloud_test_util::{
    check, check_eq,
    cli::print_test_results,
    provider_test::{test_provider, Provider},
    testing::{TestOptions, TestResult},
};
#[allow(unused_imports)]
use wasmcloud_test_util::{run_selected, run_selected_spawn};

#[tokio::test]
async fn run_all() {
    let opts = TestOptions::default();
    let res = run_selected_spawn!(&opts, health_check, get_set);
    print_test_results(&res);

    let passed = res.iter().filter(|tr| tr.passed).count();
    let total = res.len();
    assert_eq!(passed, total, "{} passed out of {}", passed, total);

    // try to let the provider shut dowwn gracefully
    let provider = test_provider().await;
    let _ = provider.shutdown().await;
}

/// returns a new test key with the given prefix
/// The purpose is to make sure different tests don't collide with each other
fn new_key(prefix: &str) -> String {
    format!("{}_{:x}", prefix, rand::random::<u32>())
}

// syntactic sugar for set
async fn set<T1: ToString, T2: ToString>(
    kv: &KvDynamoDbSender<Provider>,
    ctx: &Context,
    key: T1,
    value: T2,
) -> RpcResult<()> {
    kv.set(
        ctx,
        &SetRequest {
            key: key.to_string(),
            value: value.to_string(),
            ..Default::default()
        },
    )
    .await
}

/// test that health check returns healthy
async fn health_check(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    // health check
    let hc = prov.health_check().await;
    check!(hc.is_ok())?;
    Ok(())
}

/// get and set
async fn get_set(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    tokio::time::sleep(Duration::from_secs(2)).await;

    // create client and ctx
    let kv = KvDynamoDbSender::via(prov);
    let ctx = Context::default();

    let key = new_key("get");
    const VALUE: &str = "Alice";

    let get_resp = kv.get(&ctx, &key).await?;
    check_eq!(get_resp.exists, false)?;

    set(&kv, &ctx, &key, VALUE).await?;

    let get_resp = kv.get(&ctx, &key).await?;
    check!(get_resp.exists)?;
    check_eq!(get_resp.value.as_str(), VALUE)?;

    let keys_resp = kv.keys(&ctx, &KeysRequest::default()).await?;
    check_eq!(keys_resp.len(), 1)?;
    check_eq!(keys_resp[0], key)?;

    let _ = kv.del(&ctx, &key).await?;

    let get_resp = kv.get(&ctx, &key).await?;
    check_eq!(get_resp.exists, false)?;

    log::debug!("done!!!!");

    Ok(())
}
