type Term = usize;
type CandidateID = usize;
type LogIndex = usize;

struct LogEntry {
    cmd: String,
    term: Term,
}

// Persistent state on all servers:
// (Updated on stable storage before responding to RPCs)
struct StatePersistent {
    // latest term server has seen (initialized to 0 on first boot, increases monotonically)
    current_term: Term,
    // candidateId that received vote in current term (or null if none)
    voted_for: Option<CandidateID>,
}

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
    prev_log_entries: Vec<LogEntry>,
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
