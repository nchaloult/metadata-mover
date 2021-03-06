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
        // move_artist_to_metadata(&path)?;
        move_artist_to_metadata(&path)?;
    }

    println!("All finished!");
    Ok(())
}

fn move_artist_to_metadata(filepath_str: &String) -> Result<(), &'static str> {
    let path = Path::new(filepath_str);
    let filename = path.file_stem().ok_or("failed to parse file name from \
        file path")?;

    // TODO: this isn't such a user-friendly error message...
    let filename_str = filename.to_str().ok_or("failed to convert file name \
        into string")?;

    let divider_idx = filename_str.find("-").ok_or("file name does not \
        contain \"-\" separator")?;
    let artist_name = &filename_str[..(divider_idx - 1)];

    set_artist_metadata_tag(&filepath_str, &artist_name)?;
    remove_artist_name_from_file_name(
        path, filepath_str, filename_str, divider_idx
    )?;

    Ok(())
}

fn set_artist_metadata_tag(
    filepath: &str,
    artist_name: &str,
) -> Result<(), &'static str> {
    let mut tag = Tag::new();
    tag.set_artist(artist_name);

    if let Err(e) = tag.write_to_path(filepath, Version::Id3v24) {
        println!("problem with filepath {}: {}", filepath, e);
        return Err("failed to write artist tag to mp3");
    }
    Ok(())
}

fn remove_artist_name_from_file_name(
    filepath: &Path,
    filepath_str: &str,
    filename_str: &str,
    divider_idx: usize,
) -> Result<(), &'static str> {
    let song_name = &filename_str[(divider_idx + 2)..];
    // Guaranteed to be fine since we've already unwrapped this path's file
    // stem.
    //
    // TODO: this isn't such a user-friendly error message....
    let path_parent_str = filepath.parent()
                                  .unwrap()
                                  .to_str()
                                  .ok_or("failed to convert file path parent \
                                         into string")?;
    // TODO: fix hard-coded file extension (even tho this program can only work
    // mp3s).
    let path_without_artist_str = path_parent_str.to_owned() + "/" + song_name
        + ".mp3";
    let path_without_artist = Path::new(&path_without_artist_str);

    if let Err(_) = fs::rename(filepath_str, path_without_artist) {
        return Err("failed to remove artist name from mp3 file name")
    }
    Ok(())
}
