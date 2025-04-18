pub fn compress(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut compressed = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let mut best_match_len = 0;
        let mut best_match_offset = 0;

        for j in (0..i).rev().take(4096) {
            let mut current_len = 0;
            while i + current_len < data.len() && j + current_len < i && data[i + current_len] == data[j + current_len] {
                current_len += 1;
            }

            if current_len > best_match_len {
                best_match_len = current_len;
                best_match_offset = (i - j) as u16;
            }
        }

        if best_match_len >= 2 { // Changed from 3 to 2
            compressed.push(0); // Flag for match
            compressed.push((best_match_offset >> 8) as u8);
            compressed.push((best_match_offset & 0xFF) as u8);
            compressed.push(best_match_len as u8);
            i += best_match_len;
        } else {
            compressed.push(1); // Flag for literal
            compressed.push(data[i]);
            i += 1;
        }
    }

    compressed
}

pub fn decompress(compressed: &[u8]) -> Vec<u8> {
    if compressed.is_empty() {
        return Vec::new();
    }

    let mut decompressed = Vec::new();
    let mut i = 0;

    while i < compressed.len() {
        let flag = compressed[i];
        i += 1;

        if flag == 0 { // Match
            if i + 3 >= compressed.len() {
                break;
            }
            let offset_high = compressed[i] as u16;
            let offset_low = compressed[i + 1] as u16;
            let offset = (offset_high << 8) | offset_low;
            let length = compressed[i + 2] as usize;
            i += 3;

            if offset as usize > decompressed.len() {
                break;
            }

            let start = decompressed.len() - offset as usize;
            for j in 0..length {
                decompressed.push(decompressed[start + j]);
            }
        } else { // Literal
            if i >= compressed.len() {
                break;
            }
            decompressed.push(compressed[i]);
            i += 1;
        }
    }

    decompressed
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lz_roundtrip() {
        let input = b"ABABABABABAB";
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }
}
