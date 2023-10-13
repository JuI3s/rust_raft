use rust_raft::raft_rpc::raft_rpc::*;

use crate::node::interface::*;
use crate::node::overlay_node::*;
pub struct Node<T: IOverlayNode> {
    role: Role,
    overlay_node: Box<T>,
}

impl Node<OverlayNode> {
    pub fn new() -> Self {
        Node {
            role: Role::Follower,
            overlay_node: Box::new(OverlayNode::new()),
        }
    }
}

impl INode for Node<OverlayNode> {
    fn recv_rpc(&mut self, arg: &RpcArg) -> RpcRet {
        self.overlay_node.recv_rpc(arg)
    }

    fn convert_to_candidate(&mut self) {
        self.role = Role::Candidate;
        unimplemented!();
    }

    fn convert_to_leader(&mut self) {
        self.role = Role::Leader;
        unimplemented!();
    }

    fn convert_to_follower(&mut self) {
        self.role = Role::Follower;
        unimplemented!();
    }
}
