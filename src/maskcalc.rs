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
    let output = Ipv4Addr::to_bits(mask).leading_ones() as u8;
    if Ipv4Addr::from_bits(Subnet::subnet_mask(output)) != mask {
        Err("Wrong mask input! Consider 255.255.252.0")?;
    };
    Ok(output)
}
