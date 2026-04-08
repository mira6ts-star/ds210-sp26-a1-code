use analytics_lib::{dataset::Dataset, query::Query, solution::compute_query_on_dataset};
use interface::RPCInterfaceClient;
use tarpc::context::Context;

pub async fn run_hello(rpc_client: &RPCInterfaceClient) {
    let result = rpc_client.hello(Context::current()).await.unwrap();
    println!("The server says: `{}`", result);
}

// calls server and gets back the entire dataset
// runs query on the client
pub async fn run_slow_rpc(rpc_client: &RPCInterfaceClient, query: Query) -> Dataset {
    println!("using slow_rpc");
    let dataset = rpc_client.slow_rpc(Context::current()).await.unwrap();
    compute_query_on_dataset(&dataset, &query)
}

// sends query to the server
// gets back just the result
pub async fn run_fast_rpc(rpc_client: &RPCInterfaceClient, query: Query) -> Dataset {
    println!("using fast_rpc");

    // You should call fast_rpc here and not slow_rpc.
    todo!("Implement this");
}
