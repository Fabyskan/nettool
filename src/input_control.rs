use crate::subnet::Subnet;
pub struct InputArguments {
    tool: String,
    argument: Vec<String>,
}
impl InputArguments {
    pub fn tool(&self) -> &str {
        self.tool.as_str()
    }
    pub fn argument(&self, index: usize) -> &str {
        self.argument[index].as_str()
    }
    pub fn argument_len(&self) -> usize {
        self.argument.len()
    }

    pub fn new(args: &[String]) -> Result<InputArguments, &'static str> {
        if args.len() < 2 {
            return Err("No tool given!");
        }
        let tool = args[1].clone();
        let argument = args[2..].to_vec();
        Ok(InputArguments { tool, argument })
    }
    pub fn subcalc(arguments: &InputArguments) {
        if arguments.argument_len() == 1 {
            let subnet = match Subnet::from_cidr(arguments.argument(0)) {
                Ok(subnet) => subnet,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            };
            printer(&subnet);
        };
        if arguments.argument_len() == 0 {
            eprintln!("Tool missing Arguments! Consider subcalc [IP CIDR]");
            std::process::exit(2)
        };
        if arguments.argument_len() >= 2 {
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
