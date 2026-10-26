use crate::input_control::InputArguments;
use crate::subnet::Subnet;
use std::net::Ipv4Addr;
pub fn maskcalc(arguments: &InputArguments) {
    match arguments.argument_len() {
        0 => {
            eprintln!("Tool missing Arguments! Consider maskcalc [mask/prefix]");
            std::process::exit(2)
        }
        1 => {
            if arguments.argument(0).contains(".") {
                let prefix = match from_mask_input(arguments.argument(0)) {
                    Ok(prefix) => prefix,
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(2);
                    }
                };
                println!("{}", prefix);
            } else {
                let mask = match from_prefix_input(arguments.argument(0)) {
                    Ok(mask) => mask,
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(2);
                    }
                };
                println!("{}", mask);
            }
        }
        _ => {
            eprintln!("Too many arguments!");
            std::process::exit(2)
        }
    }
}
pub fn from_prefix_input(input: &str) -> Result<Ipv4Addr, String> {
    let prefix = input
        .strip_prefix("/")
        .unwrap_or(input)
        .parse()
        .map_err(|_| "Wrong input! Consider 22 or /22")?;
    if !(0..=32).contains(&prefix) {
        return Err(String::from(
            "For prefix only values between 0 and 32 allowed!",
        ));
    }
    Ok(Ipv4Addr::from_bits(Subnet::subnet_mask(prefix)))
}
pub fn from_mask_input(input: &str) -> Result<u8, String> {
    let mask: Ipv4Addr = input
        .trim()
        .parse()
        .map_err(|_| "Wrong mask input! Consider 255.255.252.0")?;
    let output = mask.to_bits().leading_ones() as u8;
    if Ipv4Addr::from_bits(Subnet::subnet_mask(output)) != mask {
        return Err(String::from("Invalid mask! Consider 255.255.252.0"));
    };
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_mask_input_middle_zero_mask() {
        let mask = "255.0.252.0";
        assert!(from_mask_input(&mask).is_err());
    }
    #[test]
    fn from_mask_input_normal() {
        let mask = "255.255.252.0";
        assert_eq!(from_mask_input(&mask), Ok(22));
    }
    #[test]
    fn from_prefix_input_middle_invalid_prefix() {
        let prefix = "/33";
        assert!(from_prefix_input(&prefix).is_err());
    }
}
