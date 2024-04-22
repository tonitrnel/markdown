pub fn encode_char(char: char, buf: &mut [u8]) -> &[u8] {
    let code = char as u32;
    match code {
        0x10000.. => {
            buf[0] = ((code >> 18) & 0x7 | 0xf0) as u8;
            buf[1] = ((code >> 12) & 0x3f | 0x80) as u8;
            buf[2] = ((code >> 6) & 0x3f | 0x80) as u8;
            buf[3] = (code & 0x3f | 0x80) as u8;
            buf
        }
        0x800.. => {
            buf[0] = ((code >> 12) & 0xf | 0xe0) as u8;
            buf[1] = ((code >> 6) & 0x3f | 0x80) as u8;
            buf[2] = (code & 0x3f | 0x80) as u8;
            &buf[0..3]
        }
        0x80.. => {
            buf[0] = ((code >> 6) & 0x1f | 0xc0) as u8;
            buf[1] = (code & 0x3f | 0x80) as u8;
            &buf[0..2]
        }
        _ => {
            buf[0] = char as u8;
            &buf[0..1]
        }
    }
}
#[allow(unused)]
pub fn encode_text(str: &str) -> Vec<u8> {
    let chars = str.chars();
    let mut buffer = Vec::new();
    for char in chars {
        let code = char as u32;
        match code {
            0x10000.. => {
                buffer.extend_from_slice(encode_char(char, &mut [0; 4]));
            }
            0x800.. => {
                buffer.extend_from_slice(encode_char(char, &mut [0; 3]));
            }
            0x80.. => {
                buffer.extend_from_slice(encode_char(char, &mut [0; 2]));
            }
            _ => {
                buffer.extend_from_slice(encode_char(char, &mut [0; 1]));
            }
        };
    }
    buffer
}

#[allow(unused)]
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
