mod input_control;
mod subnet;
use crate::input_control::InputArguments;
use crate::subnet::Subnet;
use std::env;

fn printer(subnet: &Subnet) {
    println!("Prefix: {}", subnet.prefix());
    println!("Netaddress: {}", subnet.net_addr());
    println!("Subnet Mask: {}", subnet.mask());
    println!("First Host: {}", subnet.first_host());
    println!("Last Host: {}", subnet.last_host());
    match subnet.broadcast() {
        Some(broadcast) => println!("Broadcast: {}", broadcast),
        None => println!("No broadcast defined!"),
    };

    println!("Total Hosts: {}", subnet.total_hosts());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = InputArguments::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(2);
    });

    match arguments.tool() {
        "subcalc" if InputArguments::argument_len(&arguments) == 1 => {
            let subnet = match Subnet::from_cidr(arguments.argument(0)) {
                Ok(subnet) => subnet,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            };
            printer(&subnet);
        }
        "subcalc" if InputArguments::argument_len(&arguments) == 0 => {
            eprintln!("Tool missing Arguments! Consider subcalc [IP CIDR]");
            std::process::exit(2)
        }
        _ if InputArguments::argument_len(&arguments) == 0 => {
            eprint!("Unknown tool choose a provided tool: \nsubcalc\n");
            std::process::exit(1)
        }
        _ => {
            eprintln!("Too many arguments!");
            std::process::exit(2)
        }
    };
}
