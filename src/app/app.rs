use rust_raft::raft_rpc::raft_rpc::RpcArg;
use tokio::sync::mpsc;

use crate::node::{
    interface::{INode, IOverlayNode},
    node::Node,
    overlay_node::OverlayNode,
};

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

    pub async fn start(&mut self) {
        println!("Raft server started.");

        let duration = tokio::time::Duration::from_millis(self.get_heartbeat_duration());
        let mut heartbeat = tokio::time::interval(duration);
        heartbeat.tick().await; // First tick is immediate

        let (tx_rpc, mut rx_rpc) = mpsc::unbounded_channel::<&RpcArg>();

        self.listen_for_rpc_calls();

        loop {
            tokio::select! {
                rpc = rx_rpc.recv() => {
                    // run another async function to process the result and break out of the loop here
                    match rpc {
                        Some(arg) => {
                            self.local_node.recv_rpc(arg);
                        },
                        None => {},
                    }
                },
                _ = heartbeat.tick() => {
                    // Run an async function on every tick while the network call is still in progress.
                    unimplemented!();
                },
            }
            // • If commitIndex > lastApplied: increment lastApplied, apply
            // log[lastApplied] to state machine (§5.3)
            // • If RPC request or response contains term T > currentTerm: set
            // currentTerm = T, convert to follower (§5.1)
        }
    }

    fn get_heartbeat_duration(&self) -> u64 {
        self.config.heartbeat_duration
    }
}
