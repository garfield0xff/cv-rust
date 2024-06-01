use std::{io::{self, BufRead, Read, Seek}, str::from_utf8};
use byteorder::{LittleEndian, ReadBytesExt};
use log::info;
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


///  BAGS 2.0 FORMAT
///  #ROSBAG V2.0
///  <record 1><record 2>....<record N> 
pub fn read_header<R: BufRead>(reader: &mut R) -> io::Result<()>{

    let mut read_bytes = Vec::new();
    reader.read_until(b'\n', &mut read_bytes).expect("failed to read buffer");
    let header = String::from_utf8_lossy(&read_bytes);

    info!("header : {}", header);

    read_bytes.clear();

    for _ in 0..10 {
        reader.read_until(b'=', &mut read_bytes).expect("failed to read buffer");
        let index_pos = String::from_utf8_lossy(&read_bytes);
        info!("{}", index_pos);
        read_bytes.clear();
    }

    Ok(())
}



