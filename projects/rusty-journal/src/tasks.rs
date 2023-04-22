use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }

    fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
        file.seek(SeekFrom::Start(0))?;

        let tasks: Vec<Task> = match serde_json::from_reader(file) {
            Ok(tasks) => tasks,
            Err(e) if e.is_eof() => Vec::new(),
            Err(e) => Err(e)?,
        };

        file.seek(SeekFrom::Start(0))?;

        Ok(tasks)
    }

    pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(journal_path)?;

        let mut tasks: Vec<Task> = Task::collect_tasks(&file)?;
        file.seek(SeekFrom::Start(0))?;
        tasks.push(task);
        serde_json::to_writer(file, &tasks)?;

        Ok(())
    }

    pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(journal_path)?;

        let mut tasks: Vec<Task> = Task::collect_tasks(&file)?;

        if task_position == 0 || task_position > tasks.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
        }

        tasks.remove(task_position - 1);
        file.set_len(0)?;
        serde_json::to_writer(file, &tasks)?;

        Ok(())
    }

    pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
        let file = OpenOptions::new().read(true).open(journal_path)?;
        let tasks = Task::collect_tasks(&file)?;

        if tasks.is_empty() {
            print!("Task list is empty");
        } else {
            let mut order: u32 = 1;

            for task in tasks {
                println!("{} : {}", order, task);
                order += 1;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_task() {
        let task = Task::new(String::from("Buy milk"));
        assert_eq!(task.text, "Buy milk");

        let now = Utc::now();
        assert!(now > task.created_at);
    }
}
