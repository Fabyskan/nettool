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

    pub fn new(args: &[String]) -> Result<InputArguments, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments!");
        }
        if args.len() > 3 {
            return Err("Too many arguments!");
        }
        let tool = args[1].clone();
        let argument = args[2..].to_vec();
        Ok(InputArguments { tool, argument })
    }
}
