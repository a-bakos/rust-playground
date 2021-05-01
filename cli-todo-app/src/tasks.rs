use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

// chrono is a good crate to use if you need to handle date and time data in
// Rust. It provides an easy API for representing a moment in time.

// text stores the task description, like "pay the bills".
// created_at stores the timestamp of the task's creation.
// No status or is_complete field because we'll represent the to-do list as a
// vector of tasks (Vec<Task>). So when a task is complete, we can simply
// remove it from the vector.
#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Tasks {
    pub fn new(text: String) -> Self {
        let created_at: DateTime<Utc> = Utc::now();
        Self { text, created_at }
    }
}

// Because we'll represent our to-do list as a vector of tasks,
// we could easily use a JSON file to persist the data. To achieve that,
// the best course of action is to use another excellent crate from the Rust
// ecosystem: serde_json.
// To get started with serializing our Task type, we'll need two crates:
// serde:
// The base crate that will enable our types to derive the Serialize and
// Deserialize traits.
// serde_json:
// The crate that will implement those traits into our chosen file
// specification format, JSON.

// the first step is to include serde_json and serde in the [dependencies]
// section of our Cargo.toml file. This time we're going to use a different
// notation to specify them because we'll need to conditionally compile some
// serde features.
