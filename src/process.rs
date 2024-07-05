use serde::Deserialize;

use std::process::{Command, Stdio};
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct Process
{
    pub full_path: String,
    pub start_command: Vec<String>,
    pub exec_on_startup: bool,
    #[serde(skip)]
    is_alive: bool,
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub expected_return: Vec<i32>,
    #[allow(dead_code)]
    pub redirect_streams: bool,
    #[allow(dead_code)]
    pub output_redirect_path: String,
    #[allow(dead_code)]
    pub should_restart: i32,
    #[allow(dead_code)]
    pub number_of_restarts: i32,
}

impl Process
{
pub fn exec(&self)
{
    let mut command = Command::new(&self.full_path)
        .arg(self.start_command.join(" "))
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
                self.is_alive = false;
            }
            else
            {
                self.is_alive = false;
                panic!("PANIC AT THE DISCO");
            }
        }
        None =>
        {
            panic!("{}: terminated unexpectedly with unknowned exit code", self.name);
        }
    };
    println!("{}", status)
}

pub fn set_is_alive(&mut self, new_value: bool)
{
    self.is_alive = new_value;
}
pub fn is_alive(&self) -> bool
{
    return self.is_alive
}
}