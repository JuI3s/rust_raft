mod node;
use node::node::Node;
use node::overlay_node::OverlayNode;
use rust_raft::raft_rpc::raft_rpc::{AppendEntriesArg, RpcArg};
use tokio::sync::mpsc;

static HEARTBEAT_INTERVAL: u64 = 500;

async fn process_task_with_heartbeat() {
    // Use an mpsc channel here as recv is cancellation safe
    let (tx, mut rx) = mpsc::unbounded_channel::<u64>();

    tokio::spawn(async move {
        // let network_call_result = rest_api.call_something().await;

        // let _ = tx.send(network_call_result);
    });

    let duration = tokio::time::Duration::from_secs(HEARTBEAT_INTERVAL);
    let mut heartbeat = tokio::time::interval(duration);
    heartbeat.tick().await; // First tick is immediate

    loop {
        tokio::select! {
            network_call_result = rx.recv() => {
                              unimplemented!();
                              // run another async function to process the result and break out of the loop here
            },
            _ = heartbeat.tick() => {
                // Run an async function on every tick while the network call is still in progress.
            },
        }
    }
}

async fn listen_for_rpc_calls() {
    unimplemented!();
}

#[tokio::main]
async fn main() {
    println!("Raft server started.");

    let duration = tokio::time::Duration::from_millis(HEARTBEAT_INTERVAL);
    let mut heartbeat = tokio::time::interval(duration);
    heartbeat.tick().await; // First tick is immediate

    let (tx_rpc, mut rx_rpc) = mpsc::unbounded_channel::<&RpcArg>();

    tokio::spawn(async move {
        listen_for_rpc_calls();
    });

    loop {
        tokio::select! {
            rpc = rx_rpc.recv() => {
                match rpc {
                    None => (),
                    Some(apendEntriesArg)  => {
                        unimplemented!();
                    },
                    Some(requestVoteArg) => {
                        unimplemented!();
                    },
                }
                // run another async function to process the result and break out of the loop here
            },
            _ = heartbeat.tick() => {
                // Run an async function on every tick while the network call is still in progress.
                unimplemented!();
            },
        }
    }
}