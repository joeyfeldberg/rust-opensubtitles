extern crate opensubtitles;

use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::io::{Result, Error, ErrorKind};

use opensubtitles::hash;
use opensubtitles::client;
use opensubtitles::file_utils;

fn get_sub_for_file(filename: &str, lang: &str) -> Result<()> {
  println!("Getting subtitle for {} for language {}", filename, lang);
  let (hash, size) = hash::get_file_hash_and_size(&filename).unwrap();
  let hash_str = hash::format_hash_in_hex(hash);
  println!("Calculated hash for {}: {} and size {}", filename, hash_str, size);
  let client = client::OpenSubtitlesClient::create_client("", "", "en", "OSTestUserAgent").ok().unwrap();
  let subs: Vec<_> = client.search_subtitles(&hash_str, size, lang).unwrap();

  if subs.is_empty() {
    return Err(Error::new(ErrorKind::NotFound, "Cannot find subtitle for file"))
  }

  let mut path = PathBuf::from(filename);
  path.set_extension("zip");
  let zip_path = path.to_str().unwrap();
  println!("Downloading {}", zip_path);
  file_utils::download_file(&subs[0].ZipDownloadLink, zip_path).ok().unwrap();

  println!("Trying to extract SRT from file {}", zip_path);
  let mut path = PathBuf::from(filename);
  path.set_extension("srt");
  let srt_path = path.to_str().unwrap();
  let saved = file_utils::extract_srt_from_file(&zip_path, &srt_path).ok().unwrap();
  println!("Done, saved as {}", saved);

  fs::remove_file(&zip_path).ok().expect("Couldn't delete zip file");

  Ok(())
}

fn get_all_movie_files(path: &str) -> Vec<String> {
  let mut res: Vec<String> = Vec::new();
  let known_files_ext = ["mkv", "avi", "mp4"];
  for entry in std::fs::read_dir(&Path::new(&path)).unwrap() {
    let entry: std::fs::DirEntry = entry.unwrap();
    if std::fs::metadata(&entry.path()).unwrap().is_dir() {
      res.extend(get_all_movie_files(&entry.path().to_str().unwrap()).into_iter());
    } else {
      let path = PathBuf::from(entry.file_name());
      if let Some(ext) = path.extension() {
        if known_files_ext.iter().any(|&x| x == ext) {
          println!("found {:?}", entry.file_name());
          res.push(entry.path().to_str().unwrap().into());
        }
      }
    }
  }

  res
}

fn main() {
  let args: Vec<_> = env::args().collect();
  if args.len() < 3 {
    println!("usage: opensubtitles [scanpath] [language]");
    std::process::exit(0);
  }


  let scanpath = &args[1];
  let lang = &args[2];

  let files = get_all_movie_files(scanpath);
  for file in files {
    let res = get_sub_for_file(&file, lang);
    if res.is_err() {
      println!("Couldn't get subtitle file for {}, {:?}", &file, res.err());
    }
  }

  println!("Done.");
  std::process::exit(0);
}
