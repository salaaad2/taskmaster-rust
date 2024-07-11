use std::collections::HashMap;
use std::{self, fs};

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
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

type CommandFn = fn(&Taskmaster, &[&str]);



impl Taskmaster
{

fn help_command(&self, args: &[&str])
{
    println!("help ! {:?}", args);
}

fn quit_command(&self, args: &[&str])
{
    println!("QUIT ! {:?}", args);
}

fn show_command(&self, args: &[&str])
{
    if args.len() == 0
    {
        return;
    }

    println!("show ! {:?}", args);
    let proc = self.processes.taskmaster_processes.get(args[0]);
    match proc
    {
        Some(p) => println!("{:#?}", p),
        None => println!("a process with the following name: [{}] does not exist.", args[0])
    }
}

fn start_inner(&self, process: &Process)
{
    if process.should_restart
    {
        for _ in 0..process.number_of_restarts
        {
            // blocking
            process.exec();
            if process.is_alive()
            {
                process.set_is_alive(false);
                return;
            }
        }
        return;
    }
    process.exec();
}

pub fn start(&self)
{
    for (_, process) in &self.processes.taskmaster_processes
    {
        if process.exec_on_startup
        {
            // should be thread::spawn(|| { start_inner(process); });
            self.start_inner(process);
        }
    }

    // command line with available items
    let mut rl = DefaultEditor::new().unwrap();

    let mut commands = HashMap::<&str, CommandFn>::new();
    commands.insert("help", Self::help_command);
    commands.insert("show", Self::show_command);
    commands.insert("quit", Self::quit_command);

    loop 
    {
        let readline = rl.readline("taskmaster $> ");
        match readline
        {
            Ok(line) =>
            {

                let parts: Vec<&str> = line.split(' ').collect();
                if parts.is_empty()
                {
                    continue;
                }

                let _ = rl.add_history_entry(line.as_str());

                let command = parts[0];
                let args = &parts[1..];
                if let Some(func) = commands.get(command)
                {
                    func(self, args);
                }
            }
            Err(ReadlineError::Interrupted) =>
            {
                // C-c
                // do nothing
            }
            Err(ReadlineError::Eof) =>
            {
                // C-d
                self.quit_command(&[""]);
                break;
            }
            Err(_) =>
            {
                break;
            }
        }
    }
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
