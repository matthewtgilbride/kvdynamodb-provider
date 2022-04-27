//! kvdynamodb-provider capability provider
//!
//!
use kvdynamodb::{
    GetResponse, KeysRequest, KeysResponse, KvDynamoDb, KvDynamoDbReceiver, SetRequest,
};
use kvdynamodb_lib::{AwsConfig, DynamoDbClient};
use std::sync::Arc;
use std::{collections::HashMap, convert::Infallible};
use tokio::sync::RwLock;
use wasmbus_rpc::provider::prelude::*;

// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(KvDynamoDbProvider::default())?;

    eprintln!("kvdynamodb-provider provider exiting");
    Ok(())
}

/// kvdynamodb-provider capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(KvDynamoDb)]
struct KvDynamoDbProvider {
    actors: Arc<RwLock<HashMap<String, DynamoDbClient>>>,
}

/// use default implementations of provider message handlers
impl ProviderDispatch for KvDynamoDbProvider {}

impl KvDynamoDbProvider {
    async fn client(&self, ctx: &Context) -> RpcResult<DynamoDbClient> {
        let actor_id = ctx
            .actor
            .as_ref()
            .ok_or_else(|| RpcError::InvalidParameter("no actor in request".to_string()))?;
        // get read lock on actor-client hashmap
        let rd = self.actors.read().await;
        let client = rd
            .get(actor_id)
            .ok_or_else(|| RpcError::InvalidParameter(format!("actor not linked:{}", actor_id)))?;
        Ok(client.clone())
    }
}

/// Handle provider control commands
/// put_link (new actor link command), del_link (remove link command), and shutdown
#[async_trait]
impl ProviderHandler for KvDynamoDbProvider {
    /// Provider should perform any operations needed for a new link,
    /// including setting up per-actor resources, and checking authorization.
    /// If the link is allowed, return true, otherwise return false to deny the link.
    async fn put_link(&self, ld: &LinkDefinition) -> RpcResult<bool> {
        let config = AwsConfig::from_values(&ld.values)?;
        let link = DynamoDbClient::new(config, Some(ld.clone())).await;

        let mut update_map = self.actors.write().await;
        update_map.insert(ld.actor_id.to_string(), link);

        Ok(true)
    }

    /// Handle notification that a link is dropped - close the connection
    async fn delete_link(&self, actor_id: &str) {
        let mut aw = self.actors.write().await;
        if let Some(conn) = aw.remove(actor_id) {
            log::info!("redis closing connection for actor {}", actor_id);
            drop(conn)
        }
    }

    /// Handle shutdown request by closing all connections
    async fn shutdown(&self) -> Result<(), Infallible> {
        let mut aw = self.actors.write().await;
        // empty the actor link data and stop all servers
        for (_, conn) in aw.drain() {
            drop(conn)
        }
        Ok(())
    }
}

/// Handle Factorial methods
#[async_trait]
impl KvDynamoDb for KvDynamoDbProvider {
    async fn get<TS: ToString + ?Sized + Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<GetResponse> {
        let client = self.client(ctx).await?;
        client.get(arg).await
    }

    async fn set(&self, ctx: &Context, arg: &SetRequest) -> RpcResult<()> {
        let client = self.client(ctx).await?;
        client.set(arg).await
    }

    async fn del<TS: ToString + ?Sized + Sync>(&self, ctx: &Context, arg: &TS) -> RpcResult<bool> {
        let client = self.client(ctx).await?;
        client.del(arg).await
    }

    async fn keys(&self, ctx: &Context, arg: &KeysRequest) -> RpcResult<KeysResponse> {
        let client = self.client(ctx).await?;
        client.keys(arg).await
    }
}
