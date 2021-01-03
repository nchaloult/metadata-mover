pub struct Config {
    pub filepaths: Vec<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("must include at least one file path");
        }

        let mut filepaths = vec![String::new(); args.len() - 1];
        filepaths.clone_from_slice(&args[1..]);

        Ok(Config { filepaths })
    }
}
