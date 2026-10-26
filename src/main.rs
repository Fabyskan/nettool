mod input_control;
mod maskcalc;
mod subnet;
mod tcp;
use crate::input_control::InputArguments;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = InputArguments::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(2);
    });

    match arguments.tool() {
        "subcalc" => {
            subnet::subcalc(&arguments);
        }
        "maskcalc" => {
            maskcalc::maskcalc(&arguments);
        }
        "tcp" => {
            tcp::tcp(&arguments);
        }
        _ => {
            eprintln!("Unknown tool choose a provided tool: \n- subcalc");
            std::process::exit(2)
        }
    };
}
