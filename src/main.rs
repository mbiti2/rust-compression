use lz::{compress, decompress};

fn main() {
    println!("Hello, world!");

    let input = b"ABABABABABAB";
    let compressed = compress(input);
    println!("{:?}", compressed);
    let decompressed = decompress(&compressed);
    println!("\n{:?}", decompressed);
}
mod lz;
mod rle;
