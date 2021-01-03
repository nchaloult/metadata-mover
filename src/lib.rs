use std::error::Error;
use std::fs;
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

            let song_name = &name[(divider + 2)..];
            let path_stem = path.parent().unwrap().to_str();
            match path_stem {
                None => return Err("failed to extract parent stem from file path"),
                Some(stem) => {
                    // TODO: Fix hard-coded extension.
                    let path_without_artist_str = stem.to_owned() + song_name + ".mp3";
                    let path_without_artist = Path::new(&path_without_artist_str);
                    let rename_result = fs::rename(filepath_as_string, path_without_artist);
                    if rename_result.is_err() {
                        return Err("failed to remove artist name from file");
                    }
                },
            }
        },
    }

    Ok(())
}
