use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};

mod reader;




fn main() -> io::Result<()> {
    pretty_env_logger::init();

    let path = "bag_file";
    let file = File::open(path)?;
    let mut file_buf = BufReader::new(file);

    reader::read_header(&mut file_buf)?;
    // println!("{:?}", header);

    Ok(())
}
