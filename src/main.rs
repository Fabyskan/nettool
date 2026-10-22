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

fn main() {
    let args: Vec<String> = env::args().collect();
    let tool = &args[1];
    let argument = &args[2];

    match tool.parse::<String>() {
        Ok(tool) if tool == "subcalc" => {
            let subnet = match Subnet::from_cidr(argument) {
                Ok(subnet) => subnet,
                Err(e) => {
                    println!("{e}");
                    return;
                }
            };
            printer(&subnet);
        }
        Ok(_) => (),
        Err(_) => (),
    };
}
