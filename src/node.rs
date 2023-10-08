use std::option;
use std::usize;

use rust_raft::raft_rpc::*;
use rust_raft::state::*;

#[derive(Debug, Clone)]
enum Role {
    Follower,
    Leader,
    Candidate,
}

pub trait INode {
    fn convert_to_candidate(&mut self);
}

pub trait IOverlayNode {
    fn recv_append_entries(&mut self, arg: AppendEntriesArg) -> AppendEntriesRet;
    fn recv_request_vote(&self, arg: RequestVoteArg) -> RequestVoteRet;
}

pub struct Node<T: IOverlayNode> {
    role: Role,
    overlay_node: Box<T>,
}

pub struct OverlayNode {
    state_persistent: StatePersistent,
    state_volatile: StateVolatile,
    state_leader_volatile: Option<StateLeaderVolatile>,
}

impl Node<OverlayNode> {
    fn new() -> Self {
        Node {
            role: Role::Follower,
            overlay_node: Box::new(OverlayNode::new()),
        }
    }
}

impl INode for Node<OverlayNode> {
    fn convert_to_candidate(&mut self) {}
}

impl OverlayNode {
    fn new() -> Self {
        OverlayNode {
            state_persistent: StatePersistent {
                current_term: 0,
                voted_for: Option::default(),
                logs: Vec::default(),
            },
            state_volatile: StateVolatile {
                commit_index: 0,
                last_applied: 0,
            },
            state_leader_volatile: Option::default(),
        }
    }

    fn contains_matching_entry(&self, index: LogIndex, term: Term) -> bool {
        // first index is 1
        if index == 0 {
             false
        } else {
            let relative_index = index - 1;
            if relative_index + 1 >= self.state_persistent.logs.len() {
                false
            } else if self.state_persistent.logs[index].term < term {
                false
            } else {
                true
            }
    
        }
    }

    fn maybe_delete_conflicting_entries(&mut self, entries: &Vec<LogEntry>) {
        // If an existing entry conflicts with a new one (same index but
        // different terms), delete the existing entry and all that follow it
        // (§5.3)
        // TODO;
    }

    fn maybe_append_new_entries(&mut self, entries: &Vec<LogEntry>) {
        // Append any new entries not already in the log
        // TODO:
    }

    fn last_known_index(&self) -> LogIndex {
        self.state_persistent.logs.len() + self.state_volatile.last_applied
    }

    fn can_grant_vote(&self, arg: &RequestVoteArg) -> bool {
        // If votedFor is null or candidateId, and candidate’s log is at least as
        // up-to-date as receiver’s log, grant vote (§5.2, §5.4)
        let result = match self.state_persistent.voted_for {
            None => true,
            Some(candidate_id) => candidate_id == arg.candidate_id,
        } && arg.last_log_index >= self.last_known_index();
        result
    }
}

impl IOverlayNode for OverlayNode {
    fn recv_request_vote(&self, arg: RequestVoteArg) -> RequestVoteRet {
        // Reply false if term < currentTerm (§5.1)
        if arg.term < self.state_persistent.current_term || !self.can_grant_vote(&arg) {
            RequestVoteRet {
                term: self.state_persistent.current_term,
                vote_granted: false,
            }
        } else {
            RequestVoteRet {
                term: self.state_persistent.current_term,
                vote_granted: true,
            }
        }
    }

    fn recv_append_entries(&mut self, arg: AppendEntriesArg) -> AppendEntriesRet {
        // Reply false if term < currentTerm (§5.1)
        // Reply false if log doesn’t contain an entry at prevLogIndex whose
        // term matches prevLogTerm (§5.3)

        if arg.term < self.state_persistent.current_term
            || !self.contains_matching_entry(arg.prev_log_index, arg.prev_log_term)
        {
            AppendEntriesRet {
                term: self.state_persistent.current_term,
                success: false,
            }
        } else {
            self.maybe_delete_conflicting_entries(&arg.entries);
            self.maybe_append_new_entries(&arg.entries);

            if arg.leader_commit > self.state_volatile.commit_index {
                self.state_volatile.commit_index =
                    std::cmp::min(arg.leader_commit, self.last_known_index());
            }

            AppendEntriesRet {
                term: self.state_persistent.current_term,
                success: true,
            }
        }
    }
}
