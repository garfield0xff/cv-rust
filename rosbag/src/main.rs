use std::{fs::File, io};

use reader::read_header;

mod reader;

fn main() -> io::Result<()> {
    let path = "dataset.bag";
    let mut file = File::open(path)?;

    let header = read_header(&mut file)?;
    println!("{:?}", header);

    Ok(())
}
