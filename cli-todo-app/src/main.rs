mod cli;
mod tasks;
use cli::{Action::*, CommandLineArgs};
use structopt::StructOpt;
use tasks::Task;

fn main() {
    // connect the Actions struct to the three public functions defined in the tasks module.

    // Get the CLI args
    // destructuring our CommandLineArgs struct into its fields, so we can pass
    // those values independently to our task-handling functions.
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Unpack the journal file
    // Because journal_file is of type Option<PathBuf>, we need to extract the
    // path to our journal file or emit a panic.
    let journal_file = journal_file.expect("failed to find journal file");

    // Perform the action
    // match each possible Action to its function, passing the required fields
    // from the enum to the functions. We call .expect at the end of the match
    // block because all functions return a Result type, which can fail.
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        Done { position } => tasks::complete_task(journal_file, position),
        List => tasks::list_tasks(journal_file),
    }
    .expect("Failed to perform action");
}
