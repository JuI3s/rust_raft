use crate::state::state::*;

pub enum RpcArg {
    RequestVoteArg(RequestVoteArg),
    AppendEntriesArg(AppendEntriesArg),
}

pub enum RpcRet {
    RequestVoteRet(RequestVoteRet),
    AppendEntriesRet(AppendEntriesRet),
}

// -------------------------------------------------------
// AppendEntries RPC
// Invoked by leader to replicate log entries (§5.3); also used as heartbeat
// (§5.2).
pub struct AppendEntriesArg {
    // leader’s term
    pub term: Term,
    // so follower can redirect clients
    pub leader_id: CandidateID,
    // index of log entry immediately preceding new ones
    pub prev_log_index: LogIndex,
    // term of prevLogIndex entry
    pub prev_log_term: Term,
    // log entries to store (empty for heartbeat; may send more than one for
    // efficiency)
    pub entries: Vec<LogEntry>,
    // leader’s commitIndex
    pub leader_commit: LogIndex,
}

pub struct AppendEntriesRet {
    // currentTerm, for leader to update itself
    pub term: Term,
    // true if follower contained entry matching prevLogIndex and prevLogTerm
    pub success: bool,
}

// -------------------------------------------------------
// RequestVotes RPC
// Invoked by candidates to gather votes (§5.2).
pub struct RequestVoteArg {
    // candidate’s term
    pub term: Term,
    // candidate requesting vote
    pub candidate_id: CandidateID,
    // index of candidate’s last log entry (§5.4)
    pub last_log_index: LogIndex,
    // term of candidate’s last log entry (§5.4)
    pub last_log_term: Term,
}

pub struct RequestVoteRet {
    // currentTerm, for candidate to update itself
    pub term: Term,
    // true means candidate received vote
    pub vote_granted: bool,
}
