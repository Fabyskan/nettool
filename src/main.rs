mod input_control;
mod subnet;
use crate::{input_control::InputArguments, subnet::Subnet};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = InputArguments::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(2);
    });

    match arguments.tool() {
        "subcalc" => {
            Subnet::subcalc(&arguments);
        }
        _ => {
            eprintln!("Unknown tool choose a provided tool: \n- subcalc\n");
            std::process::exit(1)
        }
    };
}
