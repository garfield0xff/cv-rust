use std::{io::{self, Read, Seek}, str::from_utf8};
use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Header {
    index_pos: u64,
    chunk_count: u32,
    connection_count: u32,
}

#[derive(Serialize, Debug)]
pub struct Record {
    length: u32,
    data: Vec<u8>
}

pub fn read_header<R: Read + Seek>(reader: &mut R) -> io::Result<Header> {

    let mut version_buf = [0;12];
    reader.read_exact(&mut version_buf)?;
    let version = from_utf8(&version_buf).unwrap();
    if version != "#ROSBAG V2.0" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid file format"))
    }

    let index_pos = reader.read_u64::<LittleEndian>()?;
    let chunk_count = reader.read_u32::<LittleEndian>()?;
    let connection_count = reader.read_u32::<LittleEndian>()?;


    Ok(Header {
        index_pos,
        chunk_count,
        connection_count,
    })
    
}

