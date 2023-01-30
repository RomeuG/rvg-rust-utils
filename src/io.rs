use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

pub fn file_read_bytes(file: &mut File, address: u64, n: usize) -> Result<Vec<u8>, std::io::Error> {
    let mut bytes_vec = vec![0; n];

    file.seek(SeekFrom::Start(address))?;
    let _ = file.read(&mut bytes_vec)?;

    Ok(bytes_vec)
}

pub fn file_write_bytes(file: &mut File, address: u64, bytes: &[u8]) -> Result<usize, std::io::Error> {
    file.seek(SeekFrom::Start(address))?;
    let read_bytes = file.write(bytes)?;

    Ok(read_bytes)
}
