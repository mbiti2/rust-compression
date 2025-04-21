const WINDOW_SIZE: usize = 20;

pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < data.len() {
        let mut best_offset = 0;
        let mut best_len = 0;

        for offset in 1..=WINDOW_SIZE.min(i) {
            let mut length = 0;
            while i + length < data.len() && data[i - offset + length] == data[i + length] {
                length += 1;
                if length >= 255 {
                    break;
                }
            }
            if length > best_len {
                best_offset = offset;
                best_len = length;
            }
        }

        if best_len >= 3 {
            out.push(0x01);
            out.push(best_offset as u8);
            out.push(best_len as u8);
            i += best_len;
        } else {
            out.push(0x00);
            out.push(data[i]);
            i += 1;
        }
    }
    out
}

pub fn decompress(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < data.len() {
        match data[i] {
            0x00 => {
                i += 1;
                out.push(data[i]);
                i += 1;
            }
            0x01 => {
                let offset = data[i + 1] as usize;
                let length = data[i + 2] as usize;
                let start = out.len() - offset;
                for j in 0..length {
                    out.push(out[start + j]);
                }
                i += 3;
            }
            _ => panic!("Invalid marker"),
        }
    }
    out
}
