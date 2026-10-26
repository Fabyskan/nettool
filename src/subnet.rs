use crate::input_control::InputArguments;
use std::net::Ipv4Addr;
#[derive(Debug)]
pub struct Subnet {
    net_addr: u32,
    prefix: u8,
}
impl Subnet {
    pub fn net_addr(&self) -> Ipv4Addr {
        Ipv4Addr::from_bits(self.net_addr)
    }
    pub fn prefix(&self) -> u8 {
        self.prefix
    }

    pub fn new(ip: Ipv4Addr, prefix: u8) -> Result<Self, String> {
        if !(0..=32).contains(&prefix) {
            return Err(String::from(
                "For prefix only values between 0 and 32 allowed!",
            ));
        }
        let net_addr = Self::subnet_mask(prefix) & ip.to_bits();
        Ok(Self { net_addr, prefix })
    }

    pub fn subnet_mask(prefix: u8) -> u32 {
        if prefix != 0 {
            u32::MAX << (32 - prefix)
        } else {
            0
        }
    }
    pub fn broadcast(&self) -> Option<Ipv4Addr> {
        if self.prefix >= 31 {
            None
        } else {
            Some(Ipv4Addr::from_bits(
                !Self::subnet_mask(self.prefix) | self.net_addr,
            ))
        }
    }
    pub fn mask(&self) -> Ipv4Addr {
        Ipv4Addr::from_bits(Self::subnet_mask(self.prefix))
    }
    pub fn first_host(&self) -> Ipv4Addr {
        if self.prefix == 31 || self.prefix == 32 {
            Ipv4Addr::from_bits(self.net_addr)
        } else {
            Ipv4Addr::from_bits(self.net_addr + 1)
        }
    }
    pub fn last_host(&self) -> Ipv4Addr {
        if self.prefix == 31 {
            Ipv4Addr::from_bits(self.net_addr + 1)
        } else if self.prefix == 32 {
            Ipv4Addr::from_bits(self.net_addr)
        } else {
            Ipv4Addr::from_bits(self.net_addr | (!Self::subnet_mask(self.prefix) - 1))
        }
    }
    pub fn total_hosts(&self) -> u32 {
        if self.prefix == 31 {
            2
        } else if self.prefix == 32 {
            1
        } else {
            !Self::subnet_mask(self.prefix) - 1
        }
    }
    pub fn from_cidr(input: &str) -> Result<Subnet, String> {
        let (ip, prefix) = input
            .split_once('/')
            .ok_or("Wrong input! Consider 10.0.5.77/22")?;
        let ip: Ipv4Addr = ip
            .trim()
            .parse()
            .map_err(|_| "Wrong IP input! Consider 10.0.5.77/22")?;
        let prefix: u8 = prefix
            .trim()
            .parse()
            .map_err(|_| "Wrong prefix input! Consider 10.0.5.77/22")?;
        Self::new(ip, prefix)
    }
}

pub fn subcalc(arguments: &InputArguments) {
    match arguments.argument_len() {
        0 => {
            eprintln!("Tool missing Arguments! Consider subcalc [IP CIDR]");
            std::process::exit(2)
        }
        1 => {
            let subnet = match Subnet::from_cidr(arguments.argument(0)) {
                Ok(subnet) => subnet,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(2);
                }
            };
            printer(&subnet);
        }
        _ => {
            eprintln!("Too many arguments!");
            std::process::exit(2)
        }
    }
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
            Ipv4Addr::new(10, 0, 7, 255),
            subnet.broadcast().expect("Broadcast /22")
        );
    }
    #[test]
    fn prefix_22_subnet_mask() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 22);
        assert_eq!(Ipv4Addr::new(255, 255, 252, 0), subnet.mask());
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
            Ipv4Addr::new(255, 255, 255, 255),
            subnet.broadcast().expect("Broadcast /0")
        );
    }
    #[test]
    fn prefix_0_subnet_mask() {
        let subnet = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 0);
        assert_eq!(Ipv4Addr::new(0, 0, 0, 0), subnet.mask());
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
        assert_eq!(Ipv4Addr::new(255, 255, 255, 254), subnet.mask());
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
        assert_eq!(Ipv4Addr::new(255, 255, 255, 255), subnet.mask());
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
        assert_eq!(Ipv4Addr::new(255, 255, 255, 255), subnet.mask());
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
        assert_eq!(Ipv4Addr::new(255, 255, 255, 255), subnet.mask());
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

    // Eingabetests

    #[test]
    fn from_cidr_normal_net_addr() {
        let subnet = Subnet::from_cidr("10.0.5.77/22").unwrap();
        let subnet2 = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 22);
        assert_eq!(subnet2.net_addr, subnet.net_addr);
    }
    #[test]
    fn from_cidr_normal_prefix() {
        let subnet = Subnet::from_cidr("10.0.5.77/22").unwrap();
        let subnet2 = subnet_create(Ipv4Addr::new(10, 0, 5, 77), 22);
        assert_eq!(subnet2.prefix, subnet.prefix);
    }
    #[test]
    fn from_cidr_normal() {
        let subnet = Subnet::from_cidr("10.0.5.77/22");
        assert!(subnet.is_ok());
    }

    #[test]
    fn from_cidr_newline_and_space() {
        let subnet = Subnet::from_cidr(" 10.0.5.77/22\n");
        assert!(subnet.is_ok());
    }

    #[test]
    fn from_cidr_no_prefix_no_slash() {
        let subnet = Subnet::from_cidr("10.0.5.77");
        assert!(subnet.is_err());
    }
    #[test]
    fn from_cidr_space_inside() {
        let subnet = Subnet::from_cidr("10.0. 5.77/22");
        assert!(subnet.is_err());
    }

    #[test]
    fn from_cidr_normal_no_prefix() {
        let subnet = Subnet::from_cidr("10.0.5.77/");
        assert!(subnet.is_err());
    }
    #[test]
    fn from_cidr_no_ip() {
        let subnet = Subnet::from_cidr("/22");
        assert!(subnet.is_err());
    }
    #[test]
    fn from_cidr_no_input() {
        let subnet = Subnet::from_cidr("");
        assert!(subnet.is_err());
    }
    #[test]
    fn from_cidr_gibberish() {
        let subnet = Subnet::from_cidr("gsdrtashfd");
        assert!(subnet.is_err());
    }
    #[test]
    fn from_cidr_more_slashes() {
        let subnet = Subnet::from_cidr("10/0/5/77/22");
        assert!(subnet.is_err());
    }
    #[test]
    fn from_cidr_octet_too_big() {
        let subnet = Subnet::from_cidr("10.0.5.256/22");
        assert!(subnet.is_err());
    }
    #[test]
    fn from_cidr_leading_zero() {
        let subnet = Subnet::from_cidr("10.0.5.077/22");
        assert!(subnet.is_err());
    }
    #[test]
    fn from_cidr_prefix_out_of_parsingrange() {
        let subnet = Subnet::from_cidr("10.0.5.77/789");
        assert!(subnet.is_err());
    }
    #[test]
    fn from_cidr_prefix_blocked_by_new() {
        let subnet = Subnet::from_cidr("10.0.5.77/33");
        assert!(subnet.is_err());
    }
    #[test]
    fn from_cidr_prefix_negative() {
        let subnet = Subnet::from_cidr("10.0.5.77/-1");
        assert!(subnet.is_err());
    }
    #[test]
    fn from_cidr_mask_instead_of_prefix() {
        let subnet = Subnet::from_cidr("10.0.5.77/255.255.252.0");
        assert!(subnet.is_err());
    }
    #[test]
    fn from_cidr_wrong_ip_err_msg() {
        let subnet = Subnet::from_cidr("g/22");
        assert_eq!("Wrong IP input! Consider 10.0.5.77/22", subnet.unwrap_err());
    }
    #[test]
    fn from_cidr_wrong_prefix_err_msg() {
        let subnet = Subnet::from_cidr("10.0.5.77/g");
        assert_eq!(
            "Wrong prefix input! Consider 10.0.5.77/22",
            subnet.unwrap_err()
        );
    }
    #[test]
    fn from_cidr_wrong_input_err_msg() {
        let subnet = Subnet::from_cidr("g");
        assert_eq!("Wrong input! Consider 10.0.5.77/22", subnet.unwrap_err());
    }
}
