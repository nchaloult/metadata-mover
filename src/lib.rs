use std::error::Error;

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for path in config.filepaths {
        // TODO: call some function to perform the metadata swap/rewrite.
        println!("{}", path);
    }

    Ok(())
}
