use std::io;
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
    fn from_cidr(str: &str) -> Result<Subnet, String> {
        match str.split_once('/') {
            Some(input) => {
                let (ip, prefix) = input;
                let ip = ip.trim().parse().expect("IP Parse");
                let prefix = prefix.trim().parse().expect("Prefix Parse");
                Ok(Self::new(ip, prefix).expect("Call to new from_cidr"))
            }
            None => Err(String::from("Received None in parsing")),
        }
    }
}

fn main() {
    /*     let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = match Subnet::new(addr, 22) {
            Err(msg) => {
                println!("Fehler: {msg}");
                return;
            }
            Ok(subnet) => subnet,
        };
    */
    loop {
        println!("Please input your IP in CIDR.");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let subnet = match Subnet::from_cidr(&input) {
            Err(_) => {
                println!("Wrong input! Consider xxx.xxx.xxx.xxx/xx ");
                continue;
            }
            Ok(subnet) => subnet,
        };

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
        println!("IPv4: {}", input.trim());
        println!("Anzahl Hosts: {}", subnet.total_hosts());
        break;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn subnet_create(ip: Ipv4Addr, prefix: u8) -> Subnet {
        Subnet::new(ip, prefix).expect("Testdaten müssen gültig sein.")
    }

    // 22 //
    #[test]
    fn prefix_22_first_host() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 22);
        assert_eq!(Ipv4Addr::new(10, 0, 4, 1), subnet.first_host());
    }
    #[test]
    fn prefix_22_last_host() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 22);
        assert_eq!(Ipv4Addr::new(10, 0, 7, 254), subnet.last_host());
    }
    #[test]
    fn prefix_22_broadcast() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 22);
        assert_eq!(
            Ipv4Addr::new(10, 0, 7, 255).to_bits(),
            subnet.broadcast().expect("Broadcast /22")
        );
    }
    #[test]
    fn prefix_22_subnet_mask() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 22);
        assert_eq!(
            Ipv4Addr::new(255, 255, 252, 0).to_bits(),
            Subnet::subnet_mask(subnet.prefix)
        );
    }
    #[test]
    fn prefix_22_net_address() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 22);
        assert_eq!(Ipv4Addr::new(10, 0, 4, 0).to_bits(), subnet.net_addr);
    }
    #[test]
    fn prefix_22_total_hosts() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 22);
        assert_eq!(1022, subnet.total_hosts());
    }

    // 0 //

    #[test]
    fn prefix_0_first_host() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 0);
        assert_eq!(Ipv4Addr::new(0, 0, 0, 1), subnet.first_host());
    }
    #[test]
    fn prefix_0_last_host() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 0);
        assert_eq!(Ipv4Addr::new(255, 255, 255, 254), subnet.last_host());
    }
    #[test]
    fn prefix_0_broadcast() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 0);
        assert_eq!(
            Ipv4Addr::new(255, 255, 255, 255).to_bits(),
            subnet.broadcast().expect("Broadcast /0")
        );
    }
    #[test]
    fn prefix_0_subnet_mask() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 0);
        assert_eq!(
            Ipv4Addr::new(0, 0, 0, 0).to_bits(),
            Subnet::subnet_mask(subnet.prefix)
        );
    }
    #[test]
    fn prefix_0_net_address() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 0);
        assert_eq!(Ipv4Addr::new(0, 0, 0, 0).to_bits(), subnet.net_addr);
    }
    #[test]
    fn prefix_0_total_hosts() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 0);
        assert_eq!(4294967294, subnet.total_hosts());
    }

    // 31 //

    #[test]
    fn prefix_31_first_host() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 31);
        assert_eq!(Ipv4Addr::new(10, 0, 5, 76), subnet.first_host());
    }
    #[test]
    fn prefix_31_last_host() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 31);
        assert_eq!(Ipv4Addr::new(10, 0, 5, 77), subnet.last_host());
    }
    #[test]
    fn prefix_31_broadcast() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 31);
        assert_eq!(None, subnet.broadcast());
    }
    #[test]
    fn prefix_31_subnet_mask() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 31);
        assert_eq!(
            Ipv4Addr::new(255, 255, 255, 254).to_bits(),
            Subnet::subnet_mask(subnet.prefix)
        );
    }
    #[test]
    fn prefix_31_net_address() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 31);
        assert_eq!(Ipv4Addr::new(10, 0, 5, 76).to_bits(), subnet.net_addr);
    }
    #[test]
    fn prefix_31_total_hosts() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 31);
        assert_eq!(2, subnet.total_hosts());
    }

    // 32 //

    #[test]
    fn prefix_32_first_host() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 32);
        assert_eq!(Ipv4Addr::new(10, 0, 5, 77), subnet.first_host());
    }
    #[test]
    fn prefix_32_last_host() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 32);
        assert_eq!(Ipv4Addr::new(10, 0, 5, 77), subnet.last_host());
    }
    #[test]
    fn prefix_32_broadcast() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 32);
        assert_eq!(None, subnet.broadcast());
    }
    #[test]
    fn prefix_32_subnet_mask() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 32);
        assert_eq!(
            Ipv4Addr::new(255, 255, 255, 255).to_bits(),
            Subnet::subnet_mask(subnet.prefix)
        );
    }
    #[test]
    fn prefix_32_net_address() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 32);
        assert_eq!(Ipv4Addr::new(10, 0, 5, 77).to_bits(), subnet.net_addr);
    }
    #[test]
    fn prefix_32_total_hosts() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 32);
        assert_eq!(1, subnet.total_hosts());
    }

    // 32 0.0.0.0

    #[test]
    fn prefix_32_first_host_0000() {
        let subnet = subnet_create(Ipv4Addr::new(0, 0, 0, 0), 32);
        assert_eq!(Ipv4Addr::new(0, 0, 0, 0), subnet.first_host());
    }
    #[test]
    fn prefix_32_last_host_0000() {
        let subnet = subnet_create(Ipv4Addr::new(0, 0, 0, 0), 32);
        assert_eq!(Ipv4Addr::new(0, 0, 0, 0), subnet.last_host());
    }
    #[test]
    fn prefix_32_broadcast_0000() {
        let subnet = subnet_create(Ipv4Addr::new(0, 0, 0, 0), 32);
        assert_eq!(None, subnet.broadcast());
    }
    #[test]
    fn prefix_32_subnet_mask_0000() {
        let subnet = subnet_create(Ipv4Addr::new(0, 0, 0, 0), 32);
        assert_eq!(
            Ipv4Addr::new(255, 255, 255, 255).to_bits(),
            Subnet::subnet_mask(subnet.prefix)
        );
    }
    #[test]
    fn prefix_32_net_address_0000() {
        let subnet = subnet_create(Ipv4Addr::new(0, 0, 0, 0), 32);
        assert_eq!(Ipv4Addr::new(0, 0, 0, 0).to_bits(), subnet.net_addr);
    }
    #[test]
    fn prefix_32_total_hosts_0000() {
        let subnet = subnet_create(Ipv4Addr::new(0, 0, 0, 0), 32);
        assert_eq!(1, subnet.total_hosts());
    }

    // 32 255.255.255.255
    #[test]
    fn prefix_32_first_host_255() {
        let subnet = subnet_create(Ipv4Addr::new(255, 255, 255, 255), 32);
        assert_eq!(Ipv4Addr::new(255, 255, 255, 255), subnet.first_host());
    }
    #[test]
    fn prefix_32_last_host_255() {
        let subnet = subnet_create(Ipv4Addr::new(255, 255, 255, 255), 32);
        assert_eq!(Ipv4Addr::new(255, 255, 255, 255), subnet.last_host());
    }
    #[test]
    fn prefix_32_broadcast_255() {
        let subnet = subnet_create(Ipv4Addr::new(255, 255, 255, 255), 32);
        assert_eq!(None, subnet.broadcast());
    }
    #[test]
    fn prefix_32_subnet_mask_255() {
        let subnet = subnet_create(Ipv4Addr::new(255, 255, 255, 255), 32);
        assert_eq!(
            Ipv4Addr::new(255, 255, 255, 255).to_bits(),
            Subnet::subnet_mask(subnet.prefix)
        );
    }
    #[test]
    fn prefix_32_net_address_255() {
        let subnet = subnet_create(Ipv4Addr::new(255, 255, 255, 255), 32);
        assert_eq!(Ipv4Addr::new(255, 255, 255, 255).to_bits(), subnet.net_addr);
    }
    #[test]
    fn prefix_32_total_hosts_255() {
        let subnet = subnet_create(Ipv4Addr::new(255, 255, 255, 255), 32);
        assert_eq!(1, subnet.total_hosts());
    }

    // Other //
    #[test]
    fn prefix_22_diff_ip_same_net_address() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 22);
        let subnet2 = subnet_create(Ipv4Addr::new(10, 0, 4, 0), 22);
        assert_eq!(subnet2.net_addr, subnet.net_addr);
    }
    #[test]
    fn prefix_33_err() {
        let addr = Ipv4Addr::new(10, 0, 5, 77);
        let subnet = Subnet::new(addr, 33);
        assert!(subnet.is_err());
    }
}
