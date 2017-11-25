pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|&v| {
            if v == 0 {
                return vec![0];
            }

            let mut v = v;
            let mut bytes = Vec::new();

            while v > 0 {
                bytes.push((v as u8 & 0x7F) | 0x80);
                v >>= 7;
            }

            bytes[0] &= 0x7F;
            bytes.reverse();
            bytes
        })
        .collect()
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    let mut values = Vec::new();
    let mut v = 0u32;
    let mut in_sequence = false;

    for &b in bytes {
        if b & 0x80 == 0 {
            v |= b as u32;
            values.push(v);

            v = 0;
            in_sequence = false;
        } else {
            if v.leading_zeros() < 7 {
                return Err("integer overflow");
            }

            v |= b as u32 & 0x7F;
            v <<= 7;

            in_sequence = true;
        }
    }

    if in_sequence {
        return Err("incomplete byte sequence");
    }

    Ok(values)
}
