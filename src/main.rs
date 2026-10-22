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
        "subcalc" => {
            let subnet = match Subnet::from_cidr(&arguments.argument(0)) {
                Ok(subnet) => subnet,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(2);
                }
            };
            printer(&subnet);
        }
        _ => {
            eprintln!("Unknown Tool!");
            std::process::exit(1)
        }
    };
}
