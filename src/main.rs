use clap::Parser;
mod taskmaster;
mod process;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args 
{
    // yaml config_file
    #[arg(short, long, default_value_t = String::from("./single.yaml"))]
    config_file: String,

    // stdout for now
    // #[arg(short, long, default_value_t = String::from("./output.log"))]
    // log_file: String,
}

fn main() 
{
    let args = Args::parse();

    println!("config_file: {}!", args.config_file);

    let taskmaster = taskmaster::Taskmaster::new(&args.config_file);

    if taskmaster.is_ok == false
    {
        panic!("");
    }
    println!("{}", taskmaster.display_config());
    taskmaster.start();
}