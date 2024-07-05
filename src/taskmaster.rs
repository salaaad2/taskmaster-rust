use std::collections::HashMap;
use std::{self, fs};

use serde_yml;
use serde::{Deserialize};

pub struct Taskmaster
{
    processes: Processes,
    config_file: String,
    is_ok: bool
}

#[derive(Deserialize, Debug)]
struct Processes
{
    #[serde(rename = "taskmaster-processes")]
    taskmaster_processes: HashMap<String, Process>
}

impl Default for Processes {
    fn default() -> Self {
        Processes {
            taskmaster_processes: HashMap::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Process
{
    name: String,
    full_path: String,
    start_command: Vec<String>,
    expected_return: Vec<i32>,
    redirect_streams: bool,
    output_redirect_path: String,
    should_restart: i32,
    number_of_restarts: i32,
    exec_on_startup: bool,
}

impl Taskmaster
{
    pub fn new(config_file: &String) -> Taskmaster
    {
        let mut default = Taskmaster
        {
            processes: Processes::default(),
            config_file: "".to_string(),
            is_ok: true
        };
        if !config_file.ends_with(".yaml")
        {
            return Taskmaster
            {
                is_ok: false,
                ..default
            };
        }

        let yaml_content_result = fs::read_to_string(config_file);
        let yaml_content = match yaml_content_result
        {
            Ok(content) =>{ default.is_ok = true; content }
            Err(e) =>
            {
                eprintln!("File does not exist or is malformed. Aborting. {e}");
                return Taskmaster{is_ok: false, ..default};
            }
        };

        let config = serde_yml::from_str::<Processes>(&yaml_content);
        println!("{:?}", config);
        return default;
    }
}