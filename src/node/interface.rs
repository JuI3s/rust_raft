use rust_raft::raft_rpc::raft_rpc::*;

#[derive(Debug, Clone)]
pub enum Role {
    Follower,
    Leader,
    Candidate,
}

pub trait INode {
    fn convert_to_candidate(&mut self);
}

pub trait IOverlayNode {
    fn recv_append_entries(&mut self, arg: &AppendEntriesArg) -> AppendEntriesRet;
    fn recv_request_vote(&self, arg: &RequestVoteArg) -> RequestVoteRet;
}
