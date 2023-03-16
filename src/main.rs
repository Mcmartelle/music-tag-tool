extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
// use std::io::Error;
use std::path::PathBuf;
// use std::process::Command;

use anyhow::Result;
use id3::{Tag, TagLike, Version};
use pest::Parser;

#[derive(Parser)]
#[grammar = "mfn.pest"]
pub struct MFNParser;

struct SongData {
    artist: Option<String>,
    year: Option<i32>,
    album: Option<String>,
    track: Option<u32>,
    title: Option<String>,
}

impl SongData {
    fn new() -> Self {
        Self {
            artist: None,
            year: None,
            album: None,
            track: None,
            title: None,
        }
    }
}

fn main() -> Result<()> {
    match fs::read_dir(".") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            paths
                .filter(|p| p.as_ref().expect("no path").path().extension().is_some())
                .map(|p| p.as_ref().expect("no path").path())
                .for_each(|path: PathBuf| -> () {
                    match path.extension().unwrap().to_str() {
                        Some("mp3") => {
                            let filename: String =
                                path.file_stem().unwrap().to_str().unwrap().to_string();
                            println!("Filename: {}", &filename);
                            let song_data: SongData = parse_filename(&filename).unwrap();
                            fill_tags(path, song_data).unwrap_or(());
                            ()
                        }
                        Some(_) => (),
                        None => (),
                    }
                });
        }
    }

    Ok(())
}

fn parse_filename(unparsed_filename: &String) -> Result<SongData> {
    // let unparsed_filename = String::from(
    //     "Glenn Gould (1973) French Suites (16) No. 3 in B Minor, BWV 814 IV. Menuett - Trio",
    // );

    let filename = MFNParser::parse(Rule::filename, &unparsed_filename)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut song_data: SongData = SongData::new();

    for part in filename.into_inner() {
        match part.as_rule() {
            Rule::artist => {
                let strang: &str = part.as_str();
                song_data.artist = Some(String::from(strang));
                println!("Artist: {}", strang);
            }
            Rule::year => {
                let strang: &str = part.as_str();
                song_data.year = Some(strang.parse()?);
                println!("Year: {}", strang);
            }
            Rule::album => {
                let strang: &str = part.as_str();
                song_data.album = Some(String::from(strang));
                println!("Album: {}", strang);
            }
            Rule::track => {
                let strang: &str = part.as_str();
                song_data.track = Some(strang.parse()?);
                println!("Track Number: {}", strang);
            }
            Rule::title => {
                let strang: &str = part.as_str();
                song_data.title = Some(String::from(strang));
                println!("Title: {}", strang);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(song_data)
}

fn fill_tags(filepath: PathBuf, song_data: SongData) -> Result<()> {
    let mut tag = Tag::read_from_path(&filepath)?;

    if let Some(artist) = tag.artist() {
        println!("artist: {}", artist);
    } else if let Some(song_data_artist) = song_data.artist {
        tag.set_artist(song_data_artist);
    }

    if let Some(year) = tag.year() {
        println!("year: {}", year);
    } else if let Some(song_data_year) = song_data.year {
        tag.set_year(song_data_year);
    }

    if let Some(album) = tag.album() {
        println!("album: {}", album);
    } else if let Some(song_data_album) = song_data.album {
        tag.set_album(song_data_album);
    }

    if let Some(track) = tag.track() {
        println!("track: {}", track);
    } else if let Some(song_data_track) = song_data.track {
        tag.set_track(song_data_track);
    }

    if let Some(title) = tag.title() {
        println!("title: {}", title);
    } else if let Some(song_data_title) = song_data.title {
        tag.set_title(song_data_title);
    }

    tag.write_to_path(filepath, Version::Id3v24)?;

    Ok(())
}
