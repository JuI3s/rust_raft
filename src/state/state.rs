pub type Term = usize;
pub type CandidateID = usize;
pub type LogIndex = usize;
pub type LogQueue = Vec<LogEntry>;

// • If two entries in different logs have the same index and term, then they
// store the same command.
//  • If two entries in different logs have the same
// index and term, then the logs are identical in all preceding entries.
//
// The first property follows from the fact that a leader creates at most one
// entry with a given log index in a given term, and log entries never change
// their position in the log.
// The second property is guaranteed by a simple consistency check performed
// by AppendEntries. When sending an AppendEntries RPC, the leader includes
// the index and term of the entry in its log that immediately precedes the new
// entries. If the follower does not find an entry in its log with the same
// index and term, then it refuses the new entries. The consistency check acts
// as an induction step.
#[derive(Debug, Clone)]
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

impl StatePersistent {
    pub fn get_log_at_index(&self, idx: usize) -> Option<&LogEntry> {
        if idx < self.logs.len() {
            Some(&self.logs[idx])
        } else {
            Option::default()
        }
    }

    pub fn remove_log_entries_from_idx(&mut self, idx: usize) {
        let final_len = self.logs.len().saturating_sub(idx);
        self.logs.truncate(final_len); 
    }

    pub fn append_log_entries(&mut self, entries: &mut Vec<LogEntry>) {
        self.logs.append(entries);
    }
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
