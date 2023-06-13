use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;

mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file"))?;

    match action {
        Add { task } => Task::add_task(journal_file, Task::new(task)),
        List => Task::list_tasks(journal_file),
        Done { position } => Task::complete_task(journal_file, position),
    }?;

    Ok(())
}
