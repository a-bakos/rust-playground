mod cli;
mod tasks;
use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use structopt::StructOpt;
use tasks::Task;

// This function will look for the user's home directory and return it in an
// Option<PathBuf> type, just like the journal_file field from our
// CommandLineArgs type:
fn find_default_journal_file() -> Option<PathBuf> {
    // we try to build the full path to our default journal file. We build the
    // path by taking an Option type from the home::home_dir function output and
    // calling its map method with an anonymous function that pushes the string
    // ".rusty-journal.json" to the path. If the output of home::home_dir is
    // None, no action is taken, because map will work only with a Some variant.
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
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
    // The .or_else method does the opposite of the map method: it calls the
    // function it holds only if the variant is None.
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file"))?;

    // Perform the action
    // match each possible Action to its function, passing the required fields
    // from the enum to the functions. We call .expect at the end of the match
    // block because all functions return a Result type, which can fail.
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        Done { position } => tasks::complete_task(journal_file, position),
        List => tasks::list_tasks(journal_file),
    }?;

    Ok(())

    // Run the program:
    // start by calling cargo run -- to ensure that all the arguments passed
    // after -- will be sent to our program and not to cargo itself.
    //
    // $ cargo run -- -j test-journal.json add "buy milk"
    // $ cargo run -- -j test-journal.json add "take the dog for a walk"
    // $ cargo run -- -j test-journal.json add "water the plants"
    // $ cargo run -- -j test-journal.json list
    // $ cargo run -- -j test-journal.json done 2
    // $ cargo run -- -j test-journal.json list

    // It's common for some command-line applications to put their user-owned
    // files, like dotfiles and config files, in their home directories. So we
    // might want to place our default journal file there too.
    // Because home directories vary depending on the user's operating system,
    // we'll rely on a third-party crate called home to determine the directory.
}
