mod raft_rpc;
mod state;

use std::usize;

use raft_rpc::*;
use state::*;

#[derive(Debug, Clone)]
enum Role {
    Follower,
    Leader,
    Candidate,
}

struct Node {
    role: Role,
    state_persistent: StatePersistent,
    state_volatile: StateVolatile,
    state_leader_volatile: Option<StateLeaderVolatile>,
}

impl Node {
    fn new() -> Self {
        Node {
            role: Role::Follower,
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
        let relative_index = index - 1;
        if relative_index < 0 || relative_index + 1 >= self.state_persistent.logs.len() {
            false
        } else if self.state_persistent.logs[index].term < term {
            false
        } else {
            true
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

    fn last_log_index(&self) -> LogIndex {
        self.state_persistent.logs.len() + 1
    }

    pub fn recv_append_entries(&mut self, arg: AppendEntriesArg) -> AppendEntriesRet {
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
                    std::cmp::min(arg.leader_commit, self.last_log_index());
            }

            AppendEntriesRet {
                term: self.state_persistent.current_term,
                success: true,
            }
        }
    }

    pub fn recv_request_vote(&mut self, arg: RequestVoteArg) -> RequestVoteRet {
        // ToDO
        RequestVoteRet {
            term: 0,
            vote_granted: false,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
