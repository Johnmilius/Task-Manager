use crate::task::Task;
use std::fs::{self, File}; // Add File import
use std::error::Error;
use serde_json;

pub fn load_tasks(filename: &str) -> Result<Vec<Task>, Box<dyn::std::error::Error>>{

    let file =  File::open(filename)?;
    let tasks_data: Vec<Task> = serde_json::from_reader(file)?;

    Ok(tasks_data)
}

pub fn save_tasks(tasks: &Vec<Task>, filename: &str) -> Result<bool, Box<dyn::std::error::Error>>{

    let json_string = serde_json::to_string_pretty(&tasks)?;
    fs::write(filename, json_string)?;

    Ok(true)
}