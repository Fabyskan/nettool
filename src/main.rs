use std::net::Ipv4Addr;

#[derive(Debug)]
struct Subnet {
    net_addr: u32,
    prefix: u8,
}
impl Subnet {
    fn new(ip: Ipv4Addr, prefix: u8) -> Result<Self, String> {
        if !(0..=32).contains(&prefix) {
            return Err(String::from("Only values between 0 and 32 allowed!"));
        }
        let net_addr = Self::subnet_mask(prefix) & ip.to_bits();
        Ok(Self { net_addr, prefix })
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
    let subnet1 = match Subnet::new(addr, 33) {
        Err(msg) => {
            println!("Fehler: {msg}");
            return;
        }
        Ok(prefix) => prefix,
    };

    // PRINTBLOCK //
    println!("Prefix: {}", subnet1.prefix);
    println!("Netzadresse: {}", Ipv4Addr::from_bits(subnet1.net_addr));
    println!(
        "Subnetzmaske: {}",
        Ipv4Addr::from_bits(Subnet::subnet_mask(subnet1.prefix))
    );
    println!(
        "Subnetzmaske binary: {:032b}",
        Subnet::subnet_mask(subnet1.prefix)
    );
    println!("First Host: {}", subnet1.first_host());
    println!("Last Host: {}", subnet1.last_host());
    println!("Broadcast: {}", Ipv4Addr::from_bits(subnet1.broadcast()));
    println!("IPv4: {addr}");
    println!("Anzahl Hosts: {}", subnet1.total_hosts());
}
