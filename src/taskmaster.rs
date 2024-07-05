use std::collections::HashMap;
use std::{self, fs};

use serde_yml;
use serde::Deserialize;

use crate::process::Process;

pub struct Taskmaster
{
    processes: Processes,
    pub is_ok: bool,

    #[allow(dead_code)]
    config_file: String
}

impl Taskmaster
{
pub fn start(&self)
{
    self.processes.taskmaster_processes.values()
        .filter(|val| val.exec_on_startup == true)
        .for_each(|val| val.exec());
}

pub fn new(config_file: &String) -> Taskmaster
{
    let mut default = Taskmaster
    {
        processes: Processes::default(),
        config_file: "".to_string(),
        is_ok: true
    };

    let yaml_content_result =
        fs::read_to_string(config_file);
    let yaml_content = match yaml_content_result
    {
        Ok(content) => { default.is_ok = true; content }
        Err(e) =>
        {
            eprintln!("File does not exist or is malformed. Aborting. {e}");
            return Taskmaster{is_ok: false, ..default};
        }
    };

    let config = 
        serde_yml::from_str::<Processes>(&yaml_content);
    default.processes = match config
    {
        Ok(content) => content,
        Err(e) =>
        {
            eprintln!("Error parsing yaml file. aborting. {e}.");
            return default;
        }
    };
    return default;
}

pub fn display_config(&self) -> String
{
    let mut result = String::new();
    for (key, val) in &self.processes.taskmaster_processes
    {
        let formatted = format!("{}: {} [{:?}]\n", key, val.full_path, val.start_command);
        result.push_str(&formatted);
    }
    result
}
}

#[derive(Deserialize, Debug)]
struct Processes
{
    #[serde(rename = "taskmaster-processes")]
    taskmaster_processes: HashMap<String, Process>
}

impl Default for Processes
{
fn default() -> Self
{
    Processes 
    {
        taskmaster_processes: HashMap::new(),
    }
}
}
