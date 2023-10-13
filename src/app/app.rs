use rust_raft::raft_rpc::raft_rpc::RpcArg;
use tokio::sync::mpsc;

use crate::node::{node::Node, overlay_node::OverlayNode};

#[derive(Debug, Clone)]
pub struct Config {
    pub heartbeat_duration: u64,
}

pub struct App {
    pub config: Config,
    pub local_node: Node<OverlayNode>,
}

impl App {
    pub fn new(config: Config) -> App {
        let node = Node::<OverlayNode>::new();
        App {
            config: config,
            local_node: node,
        }
    }

    fn listen_for_rpc_calls(&self) {
        unimplemented!();
    }

    pub async fn start(&self) {
        println!("Raft server started.");

        let duration = tokio::time::Duration::from_millis(self.get_heartbeat_duration());
        let mut heartbeat = tokio::time::interval(duration);
        heartbeat.tick().await; // First tick is immediate

        let (tx_rpc, mut rx_rpc) = mpsc::unbounded_channel::<&RpcArg>();

        self.listen_for_rpc_calls();

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

    fn get_heartbeat_duration(&self) -> u64 {
        self.config.heartbeat_duration
    }
}
