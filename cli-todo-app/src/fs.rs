use std::fs::OpenOptions;
use std::io::{BufReader, Result, Seek, SeekFrom};
use std::path::PathBuf;

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // open the file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // consume the file's contents as a vector of tasks
    let mut tasks: Vec<Tasks> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    // rewind the file after reading it
    file.seek(SeekFrom::Start(0))?;

    // write the modified task list back into the file
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {}
