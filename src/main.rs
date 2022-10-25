use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::process;
use clap::Parser;

use buffer::Buffer;
use error::DecodeError;

use crate::bit_depth::BitDepth;
use crate::chunk::{Chunk, ChunkData, ChunkType};
use crate::color_type::ColorType;
use crate::interlace_method::InterlaceMethod;

mod bit_depth;
mod buffer;
mod chunk;
mod color_type;
mod error;
mod interlace_method;


#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    file: String,

    #[arg(short='V', long)]
    verbose: bool
}
    

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    let mut reader = BufReader::new(File::open(args.file)?);
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
    let ihdr = read_chunk(&mut buffer)?;
    println!("{:#?}", ihdr);

    let mut curr_chunk = ihdr;
    while curr_chunk.ty != ChunkType::IEND {
        curr_chunk = read_chunk(&mut buffer)?;
        println!("{:#?}", curr_chunk);
    }

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

//https://www.w3.org/TR/2003/REC-PNG-20031110/#table51
fn read_chunk(buffer: &mut Buffer) -> Result<Chunk, DecodeError> {
    let length = buffer.read_u32()?;
    let ty: ChunkType = buffer.read_n(4)?.try_into()?;

    let data = match ty {
        ChunkType::IHDR => read_ihdr_chunk_data(buffer, length)?,
        ChunkType::IDAT => read_idat_chunk_data(buffer, length)?,
        ChunkType::IEND => ChunkData::IEND,
        #[allow(unreachable_patterns)]
        _ => todo!("other chunk types"),
    };

    let crc = buffer.read_u32()?;

    Ok(Chunk {
        length,
        ty,
        data,
        crc,
    })
}

// https://www.w3.org/TR/2003/REC-PNG-20031110/#11IHDR
fn read_ihdr_chunk_data(buffer: &mut Buffer, length: u32) -> Result<ChunkData, DecodeError> {
    if length != 13 {
        return Err(DecodeError::InvalidIHDRLength);
    }

    let width = buffer.read_u32()?;
    let height = buffer.read_u32()?;
    let bit_depth: BitDepth = buffer.read_u8()?.try_into()?;
    let color_type: ColorType = buffer.read_u8()?.try_into()?;

    // TODO: Add proper support for compression_method field
    buffer.read_u8()?;

    // TODO: Add proper support for filter_method field
    buffer.read_u8()?;

    let interlace_method: InterlaceMethod = buffer.read_u8()?.try_into()?;

    println!("- read ihdr chunk data");
    Ok(ChunkData::IHDR {
        width,
        height,
        bit_depth,
        color_type,
        compression_method: 0,
        filter_method: 0,
        interlace_method,
    })
}

fn read_idat_chunk_data(buffer: &mut Buffer, length: u32) -> Result<ChunkData, DecodeError> {
    let length: usize = length.try_into().unwrap();
    let bytes = Vec::from(buffer.read_n(length)?);

    Ok(ChunkData::IDAT(bytes))
}
