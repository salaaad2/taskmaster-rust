use serde::Deserialize;
use std::cell::RefCell;

use std::process::{Command, Stdio};
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct Process
{
    pub full_path: String,
    pub start_command: Vec<String>,
    pub exec_on_startup: bool,
    #[serde(skip)]
    is_alive: RefCell<bool>,
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub expected_return: Vec<i32>,
    #[allow(dead_code)]
    pub redirect_streams: bool,
    #[allow(dead_code)]
    pub output_redirect_path: String,
    #[allow(dead_code)]
    pub should_restart: bool,
    #[allow(dead_code)]
    pub number_of_restarts: i32,
}

impl Process
{
pub fn exec(&self)
{
    let mut command = Command::new(&self.full_path)
        .args(&self.start_command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start {self.name}");

    self.set_is_alive(true);
    let status = command.wait().expect("Failed to wait on child process");
    match status.code()
    {
        Some(code) =>
        {
            if self.expected_return.contains(&code)
            {
                let mut output_data = String::new();
                command.stdout.as_mut().unwrap().read_to_string(&mut output_data).unwrap();
                println!("{}: returned successfully with exit code: {}\nstdout:\n---\n{}\n---", 
                    self.name, code, output_data);
            }
            else
            {
                self.set_is_alive(false);
                panic!("PANIC AT THE DISCO");
            }
        }
        None =>
        {
            self.set_is_alive(false);
            panic!("{}: terminated unexpectedly with unknown exit code", self.name);
        }
    };
    println!("{}", status)
}

pub fn set_is_alive(&self, new_value: bool)
{
    *self.is_alive.borrow_mut() = new_value;
}

pub fn is_alive(&self) -> bool
{
    return *self.is_alive.borrow();
}
}