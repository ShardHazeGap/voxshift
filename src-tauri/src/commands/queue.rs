//! Batch queue state and worker coordination.
use std::collections::VecDeque;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobState {
    Queued,
    Analyzing,
    Encoding { progress: f64 },
    Done { out_bytes: u64 },
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub source: String,
    pub state: JobState,
    pub preset: String,
}

pub struct Queue {
    jobs: Mutex<VecDeque<Job>>,
}

impl Queue {
    pub fn new() -> Self { Self { jobs: Mutex::new(VecDeque::new()) } }

    pub fn push(&self, job: Job) -> usize {
        let mut g = self.jobs.lock().unwrap();
        g.push_back(job);
        g.len()
    }

    pub fn next_pending(&self) -> Option<Job> {
        let mut g = self.jobs.lock().unwrap();
        g.iter_mut().find(|j| j.state == JobState::Queued).cloned()
    }

    pub fn update(&self, id: &str, state: JobState) {
        let mut g = self.jobs.lock().unwrap();
        if let Some(j) = g.iter_mut().find(|j| j.id == id) { j.state = state; }
    }

    pub fn snapshot(&self) -> Vec<Job> {
        self.jobs.lock().unwrap().iter().cloned().collect()
    }
}

impl Default for Queue { fn default() -> Self { Self::new() } }
