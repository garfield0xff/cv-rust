use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};

mod reader;


fn main() -> io::Result<()> {
    let path = "bat_path";
    let mut file = File::open(path)?;

    let header = reader::read_header(&mut file)?;
    println!("{:?}", header);

    Ok(())
}
