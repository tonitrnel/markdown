#![allow(unused)]

pub fn encode_text(str: &str) -> Vec<u8> {
    let chars = str.chars();
    let mut buffer = Vec::new();
    for char in chars {
        let code = char as u32;
        match char as u32 {
            0x10000.. => {
                buffer.push(((code >> 18) & 0x7 | 0xf0) as u8);
                buffer.push(((code >> 12) & 0x3f | 0x80) as u8);
                buffer.push(((code >> 6) & 0x3f | 0x80) as u8);
                buffer.push((code & 0x3f | 0x80) as u8);
            }
            0x800.. => {
                buffer.push(((code >> 12) & 0xf | 0xe0) as u8);
                buffer.push(((code >> 6) & 0x3f | 0x80) as u8);
                buffer.push((code & 0x3f | 0x80) as u8);
            }
            0x80.. => {
                buffer.push(((code >> 6) & 0x1f | 0xc0) as u8);
                buffer.push((code & 0x3f | 0x80) as u8);
            }
            _ => {
                buffer.push((char) as u8);
            }
        };
    }
    buffer
}
pub fn decode_text(input: &[u8]) -> String {
    let mut output = String::new();
    let mut index = 0;
    while index < input.len() {
        let mut bytes_to_move = 1u8;
        let code_point = match input[index] >> 4 {
            0..=7 => input[index] as u32,
            12..=13 => {
                bytes_to_move = 2;
                ((input[index] as u32) & 0x1f << 6) + (input[index + 1] as u32 & 0x3f)
            }
            14 => {
                bytes_to_move = 3;
                (((input[index] as u32) & 0x0f) << 12)
                    + (((input[index + 1] as u32) & 0x3f) << 6)
                    + (input[index + 2] as u32 & 0x3f)
            }
            15 => {
                bytes_to_move = 4;
                ((input[index] as u32) & 0x07 << 18)
                    + ((input[index + 1] as u32) & 0x3f << 12)
                    + ((input[index + 2] as u32) & 0x3f << 6)
                    + (input[index + 3] as u32 & 0x3f)
            }
            _ => 0,
        };
        if let Some(ch) = char::from_u32(code_point) {
            output.push(ch);
        }
        index += bytes_to_move as usize;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::char::decode_utf16;

    #[test]
    fn it_works() {
        let bytes = encode_text("例子");
        assert_eq!(decode_text(&bytes[..]), "例子");
        println!("{:?}", bytes)
    }

    #[test]
    fn wtf() {
        let n = u32::MAX >> 2;

        println!("{}", n.leading_zeros());
        println!("{:b}", u32::MAX >> 2);
    }
}
