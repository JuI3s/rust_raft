pub type Term = usize;
pub type CandidateID = usize;
pub type LogIndex = usize;
pub type LogQueue = Vec<LogEntry>;

pub struct LogEntry {
    pub cmd: String,
    pub term: Term,
}

// Persistent state on all servers:
// (Updated on stable storage before responding to RPCs)
pub struct StatePersistent {
    // latest term server has seen (initialized to 0 on first boot, increases
    // monotonically)
    pub current_term: Term,
    // candidateId that received vote in current term (or null if none)
    pub voted_for: Option<CandidateID>,
    // log entries; each entry contains command for state machine, and term
    // when entry was received by leader (first index is 1)
    pub logs: LogQueue,
}

// Volatile state on all servers.
pub struct StateVolatile {
    // index of highest log entry known to be committed (initialized to 0,
    // increases monotonically)
    pub commit_index: LogIndex,
    // index of highest log entry applied to state machine (initialized to 0,
    // increases monotonically)
    pub last_applied: LogIndex,
}

// Volatile state on leaders:
// (Reinitialized after election)
pub struct StateLeaderVolatile {
    // for each server, index of the next log entry to send to that server
    // (initialized to leader last log index + 1)
    pub next_indices: Vec<LogIndex>,
    // for each server, index of highest log entry known to be replicated on
    // server (initialized to 0, increases monotonically).
    pub match_indices: Vec<LogIndex>,
}
