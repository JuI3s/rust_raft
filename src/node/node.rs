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
    fn convert_to_candidate(&mut self) {}
}

impl IOverlayNode for Node<OverlayNode> {
    fn recv_append_entries(&mut self, arg: &AppendEntriesArg) -> AppendEntriesRet {
        self.overlay_node.recv_append_entries(arg)
    }

    fn recv_request_vote(&self, arg: &RequestVoteArg) -> RequestVoteRet {
        self.overlay_node.recv_request_vote(arg)    
    }
}
