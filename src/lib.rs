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

        // TODO: validate each file path.

        Ok(Config { filepaths })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for path in config.filepaths {
        move_artist_to_metadata(&path)?;
    }

    Ok(())
}

fn move_artist_to_metadata(filepath: &String) -> Result<(), Box<dyn Error>> {
    // TODO: Parse artist name from file name.
    // TODO: Remove artist name from file name.
    // TODO: Write/overwrite the artist metadata tag.

    println!("{}", filepath);

    Ok(())
}
