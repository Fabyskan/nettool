use std::net::Ipv4Addr;

fn main() {
    let prefix: u8 = 22; // /22
    let hostbits: u8 = 32 - prefix;
    let subnet_mask: u32 = u32::MAX << hostbits;
    let addr = Ipv4Addr::new(10, 0, 5, 77); // 10.0.5.77
    let addr_bits: u32 = addr.to_bits();
    let net_addr = subnet_mask & addr_bits;
    let broadcast: u32 = !subnet_mask | addr_bits;
    let first_host = Ipv4Addr::from_bits(net_addr + 1);
    let last_host = Ipv4Addr::from_bits(broadcast - 1);
    let total_hosts: u32 = (1 << hostbits) - 2;
    println!("IPv4: {addr}");
    println!("IPv4 binary: {:032b}", addr_bits);
    println!("Prefix: {prefix}");
    println!("Hostbits: {hostbits}");
    println!("Subnetzmaske: {}", Ipv4Addr::from_bits(subnet_mask));
    println!("Subnetzmaske binary: {:032b}", subnet_mask);
    println!("Netzadresse: {}", Ipv4Addr::from_bits(net_addr));
    println!("Broadcastadresse: {}", Ipv4Addr::from_bits(broadcast));
    println!("Erste Hostadresse: {first_host}");
    println!("Letzte Hostadresse: {last_host}");
    println!("Anzahl Hosts: {total_hosts}");
}
