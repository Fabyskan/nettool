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
            return Err("No tool given! Choose a provided tool: \n- subcalc");
        }
        let tool = args[1].clone();
        let argument = args[2..].to_vec();
        Ok(InputArguments { tool, argument })
    }
}
