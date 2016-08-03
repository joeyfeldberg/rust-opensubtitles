use std::fs::{File, OpenOptions};
use std::io::Result as IOResult;
use std::path::Path;
use std::io;
use zip::ZipArchive;
use hyper::Client;


pub fn download_file<'a>(url: &str, path: &'a str) -> IOResult<&'a str> {
  let client = Client::new();
  let mut res = client.get(url).send().unwrap();
  let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path).unwrap();
  io::copy(&mut res, &mut file).ok();
  Ok(path)
}

fn find_srt_index_in_zip(zip_file: &mut ZipArchive<io::BufReader<File>>) -> Option<usize> {
  for i in 0..zip_file.len() {
      let file = zip_file.by_index(i).unwrap();
      let p = Path::new(file.name());
      let ext = p.extension().unwrap();
      if ext == "srt" {
        return Some(i)
      }
    }

    None
}

pub fn extract_srt_from_file<'a>(zip_path: &str, srt_path: &'a str) -> IOResult<&'a str> {
  let file = try!(File::open(zip_path));
  let reader = io::BufReader::new(file);
  let mut zip = try!(ZipArchive::new(reader));

  let index = find_srt_index_in_zip(&mut zip).unwrap();
  let mut file = zip.by_index(index).unwrap();
  let mut out = OpenOptions::new().write(true).create(true)
                    .truncate(true).open(srt_path).unwrap();
  io::copy(&mut file, &mut out).ok();
  Ok(srt_path)
}
