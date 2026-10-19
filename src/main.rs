use std::net::Ipv4Addr;

#[derive(Debug)]
struct Subnet {
    net_addr: u32,
    prefix: u8,
}
impl Subnet {
    fn new(ip: Ipv4Addr, prefix: u8) -> Self {
        let net_addr = Self::subnet_mask(prefix) & ip.to_bits();
        Self { net_addr, prefix }
    }
    fn subnet_mask(prefix: u8) -> u32 {
        if prefix != 0 {
            u32::MAX << (32 - prefix)
        } else {
            0
        }
    }
    fn broadcast(&self) -> u32 {
        !Self::subnet_mask(self.prefix) | self.net_addr
    }
    fn first_host(&self) -> Ipv4Addr {
        Ipv4Addr::from_bits(self.net_addr + 1)
    }
    fn last_host(&self) -> Ipv4Addr {
        Ipv4Addr::from_bits(self.broadcast() - 1)
    }
    fn total_hosts(&self) -> u32 {
        !Self::subnet_mask(self.prefix) - 1
    }
}

fn main() {
    let addr = Ipv4Addr::new(10, 0, 5, 77);

    let addr1 = Subnet::new(addr, 22);

    println!("Prefix: {}", addr1.prefix);
    println!("Netzadresse: {}", Ipv4Addr::from_bits(addr1.net_addr));
    println!(
        "Subnetzmaske: {}",
        Ipv4Addr::from_bits(Subnet::subnet_mask(22))
    );
    println!("Subnetzmaske binary: {:032b}", Subnet::subnet_mask(22));
    println!("First Host: {}", addr1.first_host());
    println!("Last Host: {}", addr1.last_host());
    println!(
        "Broadcast aus Struct: {}",
        Ipv4Addr::from_bits(Subnet::broadcast(&addr1))
    );
    println!("IPv4: {addr}");
    println!("Anzahl Hosts: {}", addr1.total_hosts());
}
