use clap::Parser;
use decoder::Decoder;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::process;

mod decoder;
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

    #[arg(short = 'V', long)]
    verbose: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut reader = BufReader::new(File::open(args.file)?);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    let mut decoder = Decoder::new(buffer);

    if let Err(err) = decoder.decode() {
        eprintln!("Failed to decode PNG image. Reason: {:?}", err);
        process::exit(1);
    }

    Ok(())
}
