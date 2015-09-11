use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Result, Error, ErrorKind, Seek, SeekFrom};
use std::fs::File;

const MIN_CHUNKSIZE: u64 = 65536; // 64k

fn get_hash_for_chunk(file: &mut File) -> u64 {
  let mut hash: u64 = 0;
  for _ in 0..MIN_CHUNKSIZE/8 {
    let tmp: u64 =  file.read_u64::<LittleEndian>().unwrap();
    hash = hash.wrapping_add(tmp);
  }

  hash
}

pub fn get_file_hash_and_size(filename: &str) -> Result<(u64, u64)> {
  let mut file = try!(File::open(filename));
  let size = try!(file.metadata()).len();
  if size < MIN_CHUNKSIZE as u64 {
    return Err(Error::new(ErrorKind::InvalidData,
      format!("File is too small (should be at least bigger then {} bytes)", MIN_CHUNKSIZE)))
  }

  let mut hash: u64 = size;
  hash = hash.wrapping_add(get_hash_for_chunk(&mut file));
  try!(file.seek(SeekFrom::Start(size - MIN_CHUNKSIZE)));
  hash = hash.wrapping_add(get_hash_for_chunk(&mut file));

  Ok((hash, size))
}

pub fn format_hash_in_hex(hash: u64) -> String {
  format!("{:016x}", hash)
}
