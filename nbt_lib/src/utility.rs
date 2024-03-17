use std::io::Read;

pub fn read_byte(reader: &mut impl Read) -> Result<u8, ()> {
    let mut data = [0;1];
    match reader.read_exact(&mut data) {
        Ok(_) => Ok(data[0]),
        Err(_) => Err(()),
    }
}
pub fn read_char(reader: &mut impl Read) -> Result<Vec<u8>, ()> {
    let first_byte = read_byte(reader)?;
    match first_byte {
        0b0000_0000..=0b0111_1111 => Ok(vec![first_byte]),
        0b1100_0000..=0b1101_1111 => Ok(vec![first_byte, read_byte(reader)?]),
        0b1110_0000..=0b1110_1111 => Ok(vec![first_byte, read_byte(reader)?, read_byte(reader)?]),
        _ => Err(()),
    }
}
pub fn rust_string_to_mutf8_bytes(input: &str) -> Vec<u8> {
    let mut result = Vec::new();

    for c in input.chars() {
        match c {
            '\u{0000}'..='\u{007F}' => {
                // ASCII characters are represented as-is
                result.push(c as u8);
            }
            '\u{0080}'..='\u{07FF}' => {
                // Characters in the range U+0080 to U+07FF are encoded as two bytes
                result.push(((c as u32 >> 6) & 0x1F) as u8 | 0xC0);
                result.push((c as u32 & 0x3F) as u8 | 0x80);
            }
            '\u{0800}'..='\u{FFFF}' => {
                // Characters in the range U+0800 to U+FFFF are encoded as three bytes
                result.push(((c as u32 >> 12) & 0x0F) as u8 | 0xE0);
                result.push(((c as u32 >> 6) & 0x3F) as u8 | 0x80);
                result.push((c as u32 & 0x3F) as u8 | 0x80);
            }
            '\u{10000}'..='\u{10FFFF}' => {
                // Characters in the range U+10000 to U+10FFFF are encoded as four bytes
                result.push(((c as u32 >> 18) & 0x07) as u8 | 0xF0);
                result.push(((c as u32 >> 12) & 0x3F) as u8 | 0x80);
                result.push(((c as u32 >> 6) & 0x3F) as u8 | 0x80);
                result.push((c as u32 & 0x3F) as u8 | 0x80);
            }
        }
    }

    result
}
pub fn mutf8_bytes_to_rust_string(bytes: &[u8]) -> String {
    let mut result = String::new();
    let mut index = 0;

    while index < bytes.len() {
        let byte = bytes[index];
        let code_point = match byte {
            0x00..=0x7F => {
                // Single-byte character in the ASCII range
                index += 1;
                byte as u32
            }
            0xC0..=0xDF => {
                // Two-byte character
                let byte2 = bytes[index + 1];
                index += 2;
                ((byte as u32 & 0x1F) << 6) | (byte2 as u32 & 0x3F)
            }
            0xE0..=0xEF => {
                // Three-byte character
                let byte2 = bytes[index + 1];
                let byte3 = bytes[index + 2];
                index += 3;
                ((byte as u32 & 0x0F) << 12) | ((byte2 as u32 & 0x3F) << 6) | (byte3 as u32 & 0x3F)
            }
            0xF0..=0xF7 => {
                // Four-byte character
                let byte2 = bytes[index + 1];
                let byte3 = bytes[index + 2];
                let byte4 = bytes[index + 3];
                index += 4;
                ((byte as u32 & 0x07) << 18) | ((byte2 as u32 & 0x3F) << 12) |
                    ((byte3 as u32 & 0x3F) << 6) | (byte4 as u32 & 0x3F)
            }
            _ => {
                // Invalid byte, skip it
                index += 1;
                continue;
            }
        };

        if let Some(ch) = std::char::from_u32(code_point) {
            result.push(ch);
        } else {
            // Invalid code point, skip it
        }
    }

    result
}
