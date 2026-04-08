use analytics_lib::{dataset::Dataset, query::Query};

pub fn hello() -> String {
    println!("hello called");
    return String::from("hello");
}

// clones the entire dataset and sents it back
// slow bc potentially sending thousands of rows over the network for no reason
pub fn slow_rpc(input_dataset: &Dataset) -> Dataset {
    println!("slow_rpc called");
    input_dataset.clone()
}

// server recieves query
// runs compute_query_on_dataset and sends back small result
// client gets a tiny dataset instead of a huge one
pub fn fast_rpc(input_dataset: &Dataset, query: Query) -> Dataset {
    println!("fast_rpc called");
    analytics_lib::solution::compute_query_on_dataset(input_dataset, &query)
}
