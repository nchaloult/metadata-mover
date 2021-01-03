use id3::{Tag, Version};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut artist_tag = Tag::new();
    artist_tag.set_artist("Me Smile");

    artist_tag.write_to_path("song.mp3", Version::Id3v24)?;
    Ok(())
}
