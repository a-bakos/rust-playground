use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::OpenOptions;
use std::io::{BufReader, Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

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

// In the Display::fmt function, we convert the DateTime<Utc> timestamp into a
// DateTime<Local> struct, so users can see the date and time the task was
// created in local time.
// Note:
// we didn't define the created_at field by using the DateTime<Local> type since
// the beginning because the chrono::serde::ts_seconds module expects DateTime
// structs to be specialized over the Utc type.
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&local).format("%F %H:%M");
        // {:<50}: a left-aligned string padded with 50 spaces.
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
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

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // Open the file
    // First, we open the file by using OpenOptions, which allows us to specify
    // some modes for operating on the file, like read, write, and create (for
    // when the file doesn't yet exist).
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    // write the modified task list back into the file
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    // return the empty tuple value inside an Ok to indicate that everything
    // went according to our plans.
    Ok(())
}

// The complete_task function is responsible for trying to remove a task from
// the to-do list, which is stored in a file. The function needs to complete
// these actions:
// - Read the file.
// - Collect existing tasks, if there are any.
// - Remove the task that's at the indicated position, if there is one.
// - Write the updated vector of tasks back into the file.
pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    // Open the file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    // Try to remove the task
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    task.remove(task_position - 1);

    // Write the modified task list back into the file
    //
    // We're not creating the journal file. It doesn't exist.
    // We're truncating the file before writing to it because we're performing
    // a removal operation. So the file will be smaller than the original. If we
    // ignored this step, the rewound cursor would stop behind the previously
    // written bytes of the file, resulting in a malformed JSON file. When we
    // truncate the file by using the file.set_len(0) operation, we ensure that
    // we're writing the bytes in a blank page.
    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    // Open the file
    let file = OpenOptions::new().read(true).open(journal_path)?;
    // Parse the file and collect the tasks
    let tasks = collect_tasks(&file)?;

    // Enumerate and display tasks, if any
    if tasks.is_empty() {
        println!("Task list is empty.");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, taks);
            order += 1;
        }
    }

    Ok(())
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; // rewind the file before

    // Consume the file's content's as a vector of tasks.
    // serde_json asks for any type that implements the Reader trait. The File
    // type implements that trait, so we just pass it as a parameter to the
    // serde_json.from_reader function while declaring that we expect to receive
    // a Vec<Task> from it.
    // Keep in mind that accessing the file system is an I/O action that can
    // fail for various reasons. So we need to consider how our program should
    // behave (and possibly recover) in some specific cases. For example,
    // serde_json will return an error when it reaches the end of a file without
    // having found anything to parse. This event will always happen in an empty
    // file, and we need to be able to recover from it.
    // To recover from specific kinds of errors, we use guards in the match
    // expression to build an empty Vec when the specific error occurs. The Vec
    // represents an empty to-do list.
    // Note that serde_json::Error can easily be converted to the std::io::Error
    // type because it implements the From trait. That makes it possible for us
    // to use the ? operator to unpack or early return them.
    let tasks = match serde_json::from_reader(file) {
        Ok(task) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    // Rewind the file after reading it
    // Because we moved the cursor to the end of the file, we need to rewind the
    // file before we write over it again. If we don't rewind the file, we'd
    // begin writing at the cursor's last position, which would cause a
    // malformed JSON file. We use the Seek trait and the SeekFrom enum from the
    // std::io module to rewind the file.
    file.seek(SeekFrom::Start(0))?; // rewind the file after
    Ok(tasks)
}
