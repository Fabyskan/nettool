use crate::subnet::*;
use std::io;
use std::net::Ipv4Addr;
mod subnet;
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
    println!("Netaddress: {}", Ipv4Addr::from_bits(subnet.net_addr()));
    println!("Subnet Mask: {}", subnet.mask());
    println!("First Host: {}", subnet.first_host());
    println!("Last Host: {}", subnet.last_host());
    match subnet.broadcast() {
        Some(broadcast) => println!("Broadcast: {}", Ipv4Addr::from_bits(broadcast)),
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
