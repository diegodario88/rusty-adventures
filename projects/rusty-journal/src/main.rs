use structopt::StructOpt;
mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn main() {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file.expect("Failed to find journal file");

    match action {
        Add { task } => Task::add_task(journal_file, Task::new(task)),
        List => Task::list_tasks(journal_file),
        Done { position } => Task::complete_task(journal_file, position),
    }
    .expect("Failed perform  action")
}
