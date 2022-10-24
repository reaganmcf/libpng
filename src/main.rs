use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::process;

use buffer::Buffer;
use error::DecodeError;

use crate::chunk::ChunkType;

mod bitdepth;
mod buffer;
mod chunk;
mod color_type;
mod error;
mod interlace_method;

const CAT_IMG_PATH: &str = "/Users/rmcf/Code/libpng/images/cat.png";
//const INVALID_IMG_PATH: &str = "/Users/rmcf/Code/libpng/images/invalid.png";

fn main() -> io::Result<()> {
    let f = File::open(CAT_IMG_PATH)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    if let Err(err) = decode(buffer) {
        eprintln!("Failed to decode PNG image. Reason: {:?}", err);
        process::exit(1);
    }

    Ok(())
}

fn decode(buf: Vec<u8>) -> Result<(), DecodeError> {
    let mut buffer = Buffer::new(buf);
    read_signature(&mut buffer)?;
    read_chunk(&mut buffer);

    Ok(())
}

const PNG_SIGNATURE: &[u8] = &[137, 80, 78, 71, 13, 10, 26, 10];
fn read_signature(buffer: &mut Buffer) -> Result<(), DecodeError> {
    let items = buffer.read_n(8)?;

    if items == PNG_SIGNATURE {
        return Ok(());
    }

    Err(DecodeError::MissingSignature)
}

fn read_chunk(buffer: &mut Buffer) -> Result<(), DecodeError> {
    let chunk_length = buffer.read_u32()?;
    println!("chunk length: {}", chunk_length);
    let chunk_type: ChunkType = buffer.read_n(4)?.try_into()?;
    println!("\t-type: {:?}", chunk_type);

    Ok(())
}

fn read_chunk_type(buffer: &mut Buffer) -> Result<(), DecodeError> {
    let chunk_type = buffer.read_n(4);

    Ok(())
}

fn read_ihdr(buffer: &mut Buffer) -> Result<(), DecodeError> {
    let ihdr_hdr = buffer.read_n(4)?;
    let width = buffer.read_u32()?;
    let height = buffer.read_u32()?;

    println!("width: {}, height: {}", width, height);

    Ok(())
}
