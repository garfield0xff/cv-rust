use std::io::{self, Read, Seek};
use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Header {
    index_pos: u64,
    chunk_count: u32,
    connection_count: u32,
}

pub fn read_header<R: Read + Seek>(reader: &mut R) -> io::Result<Header> {
    
    let index_pos = reader.read_u64::<LittleEndian>()?;
    let chunk_count = reader.read_u32::<LittleEndian>()?;
    let connection_count = reader.read_u32::<LittleEndian>()?;

    Ok(Header {
        index_pos,
        chunk_count,
        connection_count,
    })
}

