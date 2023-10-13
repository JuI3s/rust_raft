use crate::node::interface::*;
use rust_raft::raft_rpc::raft_rpc::*;
use rust_raft::state::state::*;

pub struct OverlayNode {
    state_persistent: StatePersistent,
    state_volatile: StateVolatile,
    state_leader_volatile: Option<StateLeaderVolatile>,
}

impl OverlayNode {
    pub fn new() -> Self {
        OverlayNode {
            state_persistent: StatePersistent {
                current_term: 0,
                voted_for: Option::default(),
                logs: Vec::default(),
            },
            state_volatile: StateVolatile::new(),
            state_leader_volatile: Option::default(),
        }
    }

    fn contains_matching_entry(&self, index: LogIndex, term: Term) -> bool {
        // first index is 1
        if index == 0 {
            false
        } else {
            let relative_index = index - 1;
            if relative_index + 1 >= self.state_persistent.logs.len()
                || self.state_persistent.logs[index].term < term
            {
                false
            } else {
                true
            }
        }
    }

    fn maybe_delete_conflicting_entries(&mut self, arg: &AppendEntriesArg) {
        let entries = &arg.entries;
        // If an existing entry conflicts with a new one (same index but
        // different terms), delete the existing entry and all that follow it
        // (§5.3)
        for (i, log) in entries.iter().enumerate() {
            let log_idx = arg.prev_log_index + i + 1;
            let local_log = self.state_persistent.get_log_at_index(log_idx);
            match local_log {
                None => break,
                Some(local_log) => {
                    if local_log.term != log.term {
                        self.state_persistent.remove_log_entries_from_idx(i);
                        break;
                    }
                }
            }
        }
    }

    fn maybe_append_new_entries(&mut self, arg: &AppendEntriesArg) {
        // Append any new entries not already in the log
        let entries = &arg.entries;
        for (i, _log) in entries.iter().enumerate() {
            if !self
                .state_persistent
                .get_log_at_index(i + arg.prev_log_index)
                .is_none()
            {
                self.state_persistent
                    .append_log_entries(entries[i..].to_vec().as_mut());
                break;
            }
        }
    }

    fn last_new_entry_index(&self) -> LogIndex {
        unimplemented!();
    }

    fn last_known_index(&self) -> LogIndex {
        unimplemented!();
        // self.state_persistent.logs.len() + self.state_volatile.last_applied
    }

    fn can_grant_vote(&self, arg: &RequestVoteArg) -> bool {
        // Reply false if term < currentTerm (§5.1)
        if arg.term < self.state_persistent.current_term {
            false
        } else {
            // If votedFor is null or candidateId, and candidate’s log is at least as
            // up-to-date as receiver’s log, grant vote (§5.2, §5.4)
            let result = match self.state_persistent.voted_for {
                None => true,
                Some(candidate_id) => candidate_id == arg.candidate_id,
            } && arg.last_log_index >= self.last_known_index();
            result
        }
    }

    fn write_log(&mut self, idx: usize) {
        unimplemented!();
    }
}

impl IOverlayNode for OverlayNode {
    fn recv_rpc(&mut self, arg: &RpcArg) -> RpcRet {
        match arg {
            RpcArg::AppendEntriesArg(append_entries_arg) => {
                RpcRet::AppendEntriesRet(self.recv_append_entries(append_entries_arg))
            }
            RpcArg::RequestVoteArg(request_vote_arg) => {
                RpcRet::RequestVoteRet(self.recv_request_vote(request_vote_arg))
            }
        }
    }

    fn recv_request_vote(&self, arg: &RequestVoteArg) -> RequestVoteRet {
        RequestVoteRet {
            term: self.state_persistent.current_term,
            vote_granted: self.can_grant_vote(&arg),
        }
    }

    fn recv_append_entries(&mut self, arg: &AppendEntriesArg) -> AppendEntriesRet {
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
            self.maybe_delete_conflicting_entries(arg);
            self.maybe_append_new_entries(arg);
            let write_log = self
                .state_volatile
                .maybe_incr_commit_index(arg.leader_commit, self.last_new_entry_index());
            match write_log {
                Some(last_applied_index) => {
                    self.write_log(last_applied_index);
                }
                None => {}
            }

            AppendEntriesRet {
                term: self.state_persistent.current_term,
                success: true,
            }
        }
    }
}
