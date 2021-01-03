use std::error::Error;
use std::path::Path;

use id3::{Tag, Version};

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

fn move_artist_to_metadata(filepath_as_string: &String) -> Result<(), &'static str> {
    // TODO: Parse artist name from file name.
    // TODO: Remove artist name from file name.
    // TODO: Write/overwrite the artist metadata tag.

    let path = Path::new(filepath_as_string);
    let filename = path.file_stem().unwrap().to_str();
    match filename {
        None => return Err("failed to parse file name from path"),
        Some(name) => {
            let divider = name.find('-').unwrap();
            let artist_name = &name[..(divider - 1)];

            // TODO: This should all really be in its own function.
            let mut tag = Tag::new();
            tag.set_artist(artist_name);
            let result = tag.write_to_path(filepath_as_string, Version::Id3v24);
            if result.is_err() {
                return Err("failed to write artist tag to path");
            }
        },
    }

    Ok(())
}
