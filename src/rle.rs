
pub fn compress(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut compressed = Vec::new();
    let mut current_char = data[0];
    let mut count = 1;

    for &char in data.iter().skip(1) {
        if char == current_char {
            count += 1;
        } else {
            compressed.push(current_char);
            compressed.push(count as u8);
            current_char = char;
            count = 1;
        }
    }

    compressed.push(current_char);
    compressed.push(count as u8);

    compressed
}

pub fn decompress(compressed: &[u8]) -> Vec<u8> {
    if compressed.is_empty() {
        return Vec::new();
    }

    let mut decompressed = Vec::new();
    let mut i = 0;

    while i < compressed.len() {
        if i + 1 >= compressed.len() {
            // Handle potential odd length (shouldn't happen with valid compressed data)
            break;
        }
        let char = compressed[i];
        let count = compressed[i + 1];
        for _ in 0..count {
            decompressed.push(char);
        }
        i += 2;
    }

    decompressed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_roundtrip() {
        let input = b"AAABBBCCCCCDDDDE";
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }
}
