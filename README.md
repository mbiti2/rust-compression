# Rust Compressor

A command-line compression tool implementing RLE and LZ77 algorithms in Rust.

## Building

```bash
cargo build --release
```

## Usage

```bash
# Compress using RLE
./target/release/rust-compressor compress input.txt compressed.rle --rle

# Decompress using RLE
./target/release/rust-compressor decompress compressed.rle output.txt --rle

# Compress using LZ77
./target/release/rust-compressor compress input.txt compressed.lz --lz

# Decompress using LZ77
./target/release/rust-compressor decompress compressed.lz output.txt --lz
```

## Docker Usage

```bash
# Build the image
docker build -t rust-compressor .

# Run compression
docker run -v $(pwd):/data rust-compressor compress /data/input.txt /data/output.rle --rle
```

## Testing

```bash
cargo test
```