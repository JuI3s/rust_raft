use std::usize;

type Term = usize;
type CandidateID = usize;
type LogIndex = usize;
type LogQueue = Vec<LogEntry>;

struct LogEntry {
    cmd: String,
    term: Term,
}

#[derive(Debug, Clone)]
enum Role{
    Follower,
    Leader,
    Candidate,
}

struct Node{
    role: Role,
    state_persistent: StatePersistent,
    state_volatile: StateVolatile,
    state_leader_volatile: Option<StateLeaderVolatile>,
}

impl Node {
    fn new() -> Self {
        Node {
            role: Role::Follower, 
            state_persistent: StatePersistent { current_term: 0, voted_for: Option::default(), logs: Vec::default()}, 
            state_volatile: StateVolatile { commit_index: 0, last_applied: 0 },
            state_leader_volatile: Option::default(),
        }
    }

    fn contains_matching_entry(&self, index: LogIndex, term: Term) -> bool {
        // first index is 1
        let relative_index = index - 1;
        if relative_index < 0 || relative_index + 1 >= self.state_persistent.logs.len() {
            false
        }
        else if self.state_persistent.logs[index].term < term {
            false
        } else {
            true
        }
    }

    fn maybe_delete_conflicting_entries(&mut self, entries: &Vec<LogEntry>) {
        // If an existing entry conflicts with a new one (same index but different terms), delete the existing entry and all that follow it (§5.3)
    }

    fn maybe_append_new_entries(&mut self, entries: &Vec<LogEntry> ) {
        // Append any new entries not already in the log
    }

    fn last_log_index(&self) -> LogIndex {
        self.state_persistent.logs.len() + 1
    }

    pub fn recv_append_entries(&mut self, arg: AppendEntriesArg) -> AppendEntriesRet {
        // Reply false if term < currentTerm (§5.1)
        // Reply false if log doesn’t contain an entry at prevLogIndex whose term matches prevLogTerm (§5.3)

        if arg.term < self.state_persistent.current_term || !self.contains_matching_entry(arg.prev_log_index, arg.prev_log_term) {
             AppendEntriesRet {
                term: self.state_persistent.current_term, 
                success:false 
            }
        } else {
            self.maybe_delete_conflicting_entries(&arg.entries);

            self.maybe_append_new_entries(&arg.entries);

            if arg.leader_commit > self.state_volatile.commit_index {
                self.state_volatile.commit_index = std::cmp::min(arg.leader_commit, self.last_log_index());
            }

            AppendEntriesRet {
                term: self.state_persistent.current_term, 
                success: true  
            }

        }
    }

    pub fn recv_request_vote(&mut self, arg: RequestVoteArg) -> RequestVoteRet {
        // ToDO
        RequestVoteRet { term: 0 , vote_granted: false  }
    }
}

// Persistent state on all servers:
// (Updated on stable storage before responding to RPCs)
struct StatePersistent {
    // latest term server has seen (initialized to 0 on first boot, increases monotonically)
    current_term: Term,
    // candidateId that received vote in current term (or null if none)
    voted_for: Option<CandidateID>,
    // log entries; each entry contains command for state machine, and term when entry was received by leader (first index is 1)
    logs: LogQueue
}

// Volatile state on all servers.
struct StateVolatile {
    // index of highest log entry known to be committed (initialized to 0, increases monotonically)
    commit_index: LogIndex,
    // index of highest log entry applied to state machine (initialized to 0, increases monotonically)
    last_applied: LogIndex,
}

// Volatile state on leaders:
// (Reinitialized after election)
struct StateLeaderVolatile {
    // for each server, index of the next log entry to send to that server (initialized to leader last log index + 1)
    nextIndices: Vec<LogIndex>,
    // for each server, index of highest log entry known to be replicated on server (initialized to 0, increases monotonically).
    matchIndices: Vec<LogIndex>,
}

// -------------------------------------------------------
// AppendEntries RPC
// Invoked by leader to replicate log entries (§5.3); also used as heartbeat (§5.2).
struct AppendEntriesArg {
    // leader’s term
    term: Term,
    // so follower can redirect clients
    leader_id: CandidateID,
    // index of log entry immediately preceding new ones
    prev_log_index: LogIndex,
    // term of prevLogIndex entry
    prev_log_term: Term,
    // log entries to store (empty for heartbeat; may send more than one for efficiency)
    entries: Vec<LogEntry>,
    // leader’s commitIndex
    leader_commit: LogIndex,
}

struct AppendEntriesRet {
    // currentTerm, for leader to update itself
    term: Term,
    // true if follower contained entry matching prevLogIndex and prevLogTerm
    success: bool,
}

// -------------------------------------------------------
// RequestVotes RPC
// Invoked by candidates to gather votes (§5.2).
struct RequestVoteArg {
    // candidate’s term
    term: Term,
    // candidate requesting vote
    candidate_id: CandidateID,
    // index of candidate’s last log entry (§5.4)
    last_log_index: LogIndex,
    // term of candidate’s last log entry (§5.4)
    last_log_term: Term,
}

struct RequestVoteRet {
    // currentTerm, for candidate to update itself
    term: Term,
    // true means candidate received vote
    vote_granted: bool,
}

fn main() {
    println!("Hello, world!");
}
