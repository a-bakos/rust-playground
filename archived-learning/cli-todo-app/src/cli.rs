use std::path::PathBuf;
use structopt::StructOpt;

// The point of using structopt as the argument parser is that every
// valid invocation of the command-line interface will produce a
// CommandLineArgs value. We can use these values in the program to
// invoke the specific behavior that the user wants.

// Action::Add holds a String that describes the task being added,
// like "buy milk" or "take the dog on a walk".
// Action::Done holds the number of a task that we'll mark as done.
// For example, a 2 will cross out the second task in the numbered to-do list.
// Action::List will print the task list in the terminal.
#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file.
    Add {
        /// The task description text.
        #[structopt()]
        text: String,
    },
    /// Remove an entry from the journal file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the journal file.
    List,
}

// CommandLineArgs struct holds the Action enum as a wrapper. It also holds an
// optional argument (note the Option type) named journal_file.
// This argument is for when a user wants to point to a journal file that isn't
// the default one.
// Wrapping the action and the journal_file types together allows us to apply
// the journal_file optional argument to all nested subcommands declared in the
// Action enum.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
