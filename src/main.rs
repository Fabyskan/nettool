mod subnet;
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
struct InputArguments {
    tool: String,
    argument: String,
}
impl InputArguments {
    fn new(args: &[String]) -> Result<InputArguments, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments!");
        }
        if args.len() > 3 {
            return Err("Too many arguments!");
        }
        let tool = args[1].clone();
        let argument = args[2].clone();
        Ok(InputArguments { tool, argument })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = InputArguments::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    match arguments.tool.as_str() {
        "subcalc" => {
            let subnet = match Subnet::from_cidr(&arguments.argument) {
                Ok(subnet) => subnet,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(2);
                }
            };
            printer(&subnet);
        }
        _ => eprintln!("Unknown Tool!"),
    };
}
