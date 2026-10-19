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
    fn broadcast(&self) -> Option<u32> {
        if self.prefix >= 31 {
            None
        } else {
            Some(!Self::subnet_mask(self.prefix) | self.net_addr)
        }
    }
    fn first_host(&self) -> Ipv4Addr {
        if self.prefix == 31 || self.prefix == 32 {
            Ipv4Addr::from_bits(self.net_addr)
        } else {
            Ipv4Addr::from_bits(self.net_addr + 1)
        }
    }
    fn last_host(&self) -> Ipv4Addr {
        if self.prefix == 31 {
            Ipv4Addr::from_bits(self.net_addr + 1)
        } else if self.prefix == 32 {
            Ipv4Addr::from_bits(self.net_addr)
        } else {
            Ipv4Addr::from_bits(self.net_addr | (!Self::subnet_mask(self.prefix) - 1))
        }
    }
    fn total_hosts(&self) -> u32 {
        if self.prefix == 31 {
            2
        } else if self.prefix == 32 {
            1
        } else {
            !Self::subnet_mask(self.prefix) - 1
        }
    }
}

fn main() {
    let addr = Ipv4Addr::new(10, 0, 5, 77);
    let subnet = match Subnet::new(addr, 22) {
        Err(msg) => {
            println!("Fehler: {msg}");
            return;
        }
        Ok(subnet) => subnet,
    };

    // PRINTBLOCK //
    println!("Prefix: {}", subnet.prefix);
    println!("Netzadresse: {}", Ipv4Addr::from_bits(subnet.net_addr));
    println!(
        "Subnetzmaske: {}",
        Ipv4Addr::from_bits(Subnet::subnet_mask(subnet.prefix))
    );
    println!(
        "Subnetzmaske binary: {:032b}",
        Subnet::subnet_mask(subnet.prefix)
    );
    println!("First Host: {}", subnet.first_host());
    println!("Last Host: {}", subnet.last_host());
    match subnet.broadcast() {
        Some(broadcast) => println!("Broadcast: {}", Ipv4Addr::from_bits(broadcast)),
        None => println!("No broadcast defined!"),
    };
    println!("IPv4: {addr}");
    println!("Anzahl Hosts: {}", subnet.total_hosts());
}

#[cfg(test)]
mod tests {
    use super::*;
    // 22 //
    #[test]
    fn prefix_22_first_host() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 22);
        assert_eq!(
            Ipv4Addr::new(10, 0, 4, 1),
            subnet.expect("First Host /22").first_host()
        );
    }
    #[test]
    fn prefix_22_last_host() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 22);
        assert_eq!(
            Ipv4Addr::new(10, 0, 7, 254),
            subnet.expect("Last Host /22").last_host()
        );
    }
    #[test]
    fn prefix_22_broadcast() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 22);
        assert_eq!(
            Ipv4Addr::new(10, 0, 7, 255).to_bits(),
            subnet
                .expect("Broadcast /22")
                .broadcast()
                .expect("Broadcast /22")
        );
    }
    #[test]
    fn prefix_22_subnet_mask() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 22);
        assert_eq!(
            Ipv4Addr::new(255, 255, 252, 0).to_bits(),
            Subnet::subnet_mask(subnet.unwrap().prefix)
        );
    }
    #[test]
    fn prefix_22_net_adress() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 22);
        assert_eq!(
            Ipv4Addr::new(10, 0, 4, 0).to_bits(),
            subnet.unwrap().net_addr
        );
    }
    #[test]
    fn prefix_22_total_hosts() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 22);
        assert_eq!(1022, subnet.expect("Total Hosts /22").total_hosts());
    }

    // 0 //

    #[test]
    fn prefix_0_first_host() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 0);
        assert_eq!(
            Ipv4Addr::new(0, 0, 0, 1),
            subnet.expect("First Host /0").first_host()
        );
    }
    #[test]
    fn prefix_0_last_host() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 0);
        assert_eq!(
            Ipv4Addr::new(255, 255, 255, 254),
            subnet.expect("Last Host /0").last_host()
        );
    }
    #[test]
    fn prefix_0_broadcast() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 0);
        assert_eq!(
            Ipv4Addr::new(255, 255, 255, 255).to_bits(),
            subnet
                .expect("Broadcast /0")
                .broadcast()
                .expect("Broadcast /0")
        );
    }
    #[test]
    fn prefix_0_subnet_mask() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 0);
        assert_eq!(
            Ipv4Addr::new(0, 0, 0, 0).to_bits(),
            Subnet::subnet_mask(subnet.unwrap().prefix)
        );
    }
    #[test]
    fn prefix_0_net_adress() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 0);
        assert_eq!(
            Ipv4Addr::new(0, 0, 0, 0).to_bits(),
            subnet.unwrap().net_addr
        );
    }
    #[test]
    fn prefix_0_total_hosts() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 0);
        assert_eq!(4294967294, subnet.expect("Total Hosts /0").total_hosts());
    }

    // 31 //

    #[test]
    fn prefix_31_first_host() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 31);
        assert_eq!(
            Ipv4Addr::new(10, 0, 5, 76),
            subnet.expect("First Host /31").first_host()
        );
    }
    #[test]
    fn prefix_31_last_host() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 31);
        assert_eq!(
            Ipv4Addr::new(10, 0, 5, 77),
            subnet.expect("Last Host /31").last_host()
        );
    }
    #[test]
    fn prefix_31_broadcast() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 31);
        assert_eq!(None, subnet.expect("Broadcast /31").broadcast());
    }
    #[test]
    fn prefix_31_subnet_mask() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 31);
        assert_eq!(
            Ipv4Addr::new(255, 255, 255, 254).to_bits(),
            Subnet::subnet_mask(subnet.unwrap().prefix)
        );
    }
    #[test]
    fn prefix_31_net_adress() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 31);
        assert_eq!(
            Ipv4Addr::new(10, 0, 5, 76).to_bits(),
            subnet.unwrap().net_addr
        );
    }
    #[test]
    fn prefix_31_total_hosts() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 31);
        assert_eq!(2, subnet.expect("Total Hosts /31").total_hosts());
    }

    // 32 //

    #[test]
    fn prefix_32_first_host() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(
            Ipv4Addr::new(10, 0, 5, 77),
            subnet.expect("First Host /32").first_host()
        );
    }
    #[test]
    fn prefix_32_last_host() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(
            Ipv4Addr::new(10, 0, 5, 77),
            subnet.expect("Last Host /32").last_host()
        );
    }
    #[test]
    fn prefix_32_broadcast() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(None, subnet.expect("Broadcast /32").broadcast());
    }
    #[test]
    fn prefix_32_subnet_mask() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(
            Ipv4Addr::new(255, 255, 255, 255).to_bits(),
            Subnet::subnet_mask(subnet.unwrap().prefix)
        );
    }
    #[test]
    fn prefix_32_net_adress() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(
            Ipv4Addr::new(10, 0, 5, 77).to_bits(),
            subnet.unwrap().net_addr
        );
    }
    #[test]
    fn prefix_32_total_hosts() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(1, subnet.expect("Total Hosts /32").total_hosts());
    }

    // 32 0.0.0.0

    #[test]
    fn prefix_32_first_host_0000() {
        let addr = Ipv4Addr::new(0, 0, 0, 0);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(
            Ipv4Addr::new(0, 0, 0, 0),
            subnet.expect("First Host /32").first_host()
        );
    }
    #[test]
    fn prefix_32_last_host_0000() {
        let addr = Ipv4Addr::new(0, 0, 0, 0);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(
            Ipv4Addr::new(0, 0, 0, 0),
            subnet.expect("Last Host /32").last_host()
        );
    }
    #[test]
    fn prefix_32_broadcast_0000() {
        let addr = Ipv4Addr::new(0, 0, 0, 0);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(None, subnet.expect("Broadcast /32").broadcast());
    }
    #[test]
    fn prefix_32_subnet_mask_0000() {
        let addr = Ipv4Addr::new(0, 0, 0, 0);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(
            Ipv4Addr::new(255, 255, 255, 255).to_bits(),
            Subnet::subnet_mask(subnet.unwrap().prefix)
        );
    }
    #[test]
    fn prefix_32_net_adress_0000() {
        let addr = Ipv4Addr::new(0, 0, 0, 0);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(
            Ipv4Addr::new(0, 0, 0, 0).to_bits(),
            subnet.unwrap().net_addr
        );
    }
    #[test]
    fn prefix_32_total_hosts_0000() {
        let addr = Ipv4Addr::new(0, 0, 0, 0);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(1, subnet.expect("Total Hosts /32").total_hosts());
    }

    // 32 255.255.255.255
    #[test]
    fn prefix_32_first_host_255() {
        let addr = Ipv4Addr::new(255, 255, 255, 255);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(
            Ipv4Addr::new(255, 255, 255, 255),
            subnet.expect("First Host /32").first_host()
        );
    }
    #[test]
    fn prefix_32_last_host_255() {
        let addr = Ipv4Addr::new(255, 255, 255, 255);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(
            Ipv4Addr::new(255, 255, 255, 255),
            subnet.expect("Last Host /32").last_host()
        );
    }
    #[test]
    fn prefix_32_broadcast_255() {
        let addr = Ipv4Addr::new(255, 255, 255, 255);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(None, subnet.expect("Broadcast /32").broadcast());
    }
    #[test]
    fn prefix_32_subnet_mask_255() {
        let addr = Ipv4Addr::new(255, 255, 255, 255);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(
            Ipv4Addr::new(255, 255, 255, 255).to_bits(),
            Subnet::subnet_mask(subnet.unwrap().prefix)
        );
    }
    #[test]
    fn prefix_32_net_adress_255() {
        let addr = Ipv4Addr::new(255, 255, 255, 255);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(
            Ipv4Addr::new(255, 255, 255, 255).to_bits(),
            subnet.unwrap().net_addr
        );
    }
    #[test]
    fn prefix_32_total_hosts_255() {
        let addr = Ipv4Addr::new(255, 255, 255, 255);
        let subnet = Subnet::new(addr, 32);
        assert_eq!(1, subnet.expect("Total Hosts /32").total_hosts());
    }

    // Other //
    #[test]
    fn prefix_22_diff_ip_same_net_adress() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 22);
        let addr2 = Ipv4Addr::new(10, 0, 4, 0);
        let subnet2 = Subnet::new(addr2, 22);
        assert_eq!(subnet2.unwrap().net_addr, subnet.unwrap().net_addr);
    }
    #[test]
    fn prefix_33_err() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 33);
        assert!(subnet.is_err());
    }
}
