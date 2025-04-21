use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

mod rle;
mod lz;

fn print_usage() {
    eprintln!("Usage: compress|decompress <input_file> <output_file> --rle|--lz");
    eprintln!("Options:");
    eprintln!("  --rle    Use Run-Length Encoding");
    eprintln!("  --lz     Use LZ77 compression");
    process::exit(1);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        print_usage();
    }

    let command = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];
    let algorithm = &args[4];

    if command != "compress" && command != "decompress" {
        eprintln!("Error: First argument must be either \"compress\" or \"decompress\"");
        print_usage();
    }

    if algorithm != "--rle" && algorithm != "--lz" {
        eprintln!("Error: Invalid algorithm specified");
        print_usage();
    }

    let mut input = Vec::new();
    fs::File::open(input_file)?.read_to_end(&mut input)?;

    let result = match (command.as_str(), algorithm.as_str()) {
        ("compress", "--rle") => rle::compress(&input),
        ("decompress", "--rle") => rle::decompress(&input),
        ("compress", "--lz") => lz::compress(&input),
        ("decompress", "--lz") => lz::decompress(&input),
        _ => unreachable!(),
    };

    fs::write(output_file, result)?;
    println!("Successfully {}ed {} to {}", command, input_file, output_file);
    Ok(())
}