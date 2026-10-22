mod input_control;
mod subnet;
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
            InputArguments::subcalc(&arguments);
        }
        _ if arguments.argument_len() == 0 => {
            eprint!("Unknown tool choose a provided tool: \nsubcalc\n");
            std::process::exit(1)
        }
        _ => {
            eprintln!("Invalid Input!");
            std::process::exit(2)
        }
    };
}
