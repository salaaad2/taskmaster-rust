use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Process
{
    pub full_path: String,
    pub start_command: Vec<String>,
    pub exec_on_startup: bool,
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
    println!("EXEC!!!");
}
}