mod subnet;
use crate::subnet::Subnet;
use std::io;

fn take_input() -> String {
    println!("Please input your IP in CIDR.");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

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
    loop {
        let subnet = match Subnet::from_cidr(&take_input()) {
            Err(e) => {
                println!("{e}");
                continue;
            }
            Ok(subnet) => subnet,
        };
        printer(&subnet);
        break;
    }
}
