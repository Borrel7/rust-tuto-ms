use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};


#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
	pub text: String,

	#[serde(with = "ts_seconds")]
	pub created_at: DateTime<Utc>,	
}

impl Task {
	pub fn new(text: String) -> Task {
		let created_at: DateTime<Utc> = Utc::now();
		Task { text, created_at }
	}
}

impl fmt::Display for Task {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}



pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
	// Open the file.
	let mut file = OpenOptions::new()
	.read(true)
	.write(true)
	.create(true)
	.open(journal_path)?;

	let mut tasks = collect_tasks(&file)?;

	// Write the modified task list back into the file.
	tasks.push(task);
	serde_json::to_writer(file, &tasks)?;

	Ok(())

}



pub fn complete_task(journal_path: PathBuf, take_position: usize) -> Result<()> {

	// Open the file.
	let mut file = OpenOptions::new()
	.read(true)
	.write(true)
	.open(journal_path)?;

	let mut tasks = collect_tasks(&file)?;

	//Remove the task
	if take_position == 0 || take_position > tasks.len() {
		return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
	}
	tasks.remove(take_position - 1);

	// Write the modified task list back into the file.
    file.set_len(0)?; //rewind would cause cursor behind written byte as file size becomes smaller
    serde_json::to_writer(file, &tasks)?;
    Ok(())

}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
	//Rewind file after reading
	file.seek(SeekFrom::Start(0))?;

	//File content -> vec of tasks
	let mut tasks: Vec<Task> = match serde_json::from_reader(file) {
		Ok(tasks) => tasks,
		Err(e) if e.is_eof() => Vec::new(),
		Err(e) => Err(e)?,
	};

	//Rewind file after reading
	file.seek(SeekFrom::Start(0))?;
	Ok(tasks)
}


pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
	//Open file
	let mut file = OpenOptions::new() 
	.read(true)
	.open(journal_path)?;

	let mut tasks = collect_tasks(&file)?;

	if tasks.is_empty() {
		println!("Task list is empty!");
	} else {
		println!("{:?}", tasks);
	}
	Ok(())
}