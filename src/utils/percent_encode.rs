use crate::utils::text_encoding::encode_char;

#[cfg(test)]
pub(crate) fn decode(uri: impl AsRef<str>) -> String {
    let chars = uri.as_ref().chars().collect::<Vec<_>>();
    let len = chars.len();
    let mut bytes = Vec::new();
    let mut cur = 0;
    while cur < len {
        let chr = chars[cur];
        if chr == '%' {
            if cur + 2 > len {
                panic!("URL malformed")
            }
            bytes.push(
                u8::from_str_radix(
                    chars[(cur + 1)..(cur + 3)]
                        .iter()
                        .collect::<String>()
                        .as_str(),
                    16,
                )
                .unwrap_or(0),
            );
            cur += 2;
        } else {
            bytes.push(chr as u8);
        }
        cur += 1;
    }
    String::from_utf8_lossy(&bytes[..]).to_string()
}

/// 恒等快路径：全部字符已在安全集合内（`keep_escaped` 时含 `%`）则零分配借用。
pub(crate) fn encode_cow(url: &str, keep_escaped: bool) -> std::borrow::Cow<'_, str> {
    let identity = url.bytes().all(|b| {
        matches!(b,
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9'
            | b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')'
            | b';' | b',' | b'/' | b'?' | b':' | b'@' | b'&' | b'=' | b'+' | b'$' | b'#')
            || (b == b'%' && keep_escaped)
    });
    if identity {
        std::borrow::Cow::Borrowed(url)
    } else {
        std::borrow::Cow::Owned(encode_slow(url, keep_escaped))
    }
}

pub(crate) fn encode(url: impl AsRef<str>, keep_escaped: bool) -> String {
    match encode_cow(url.as_ref(), keep_escaped) {
        std::borrow::Cow::Borrowed(s) => s.to_string(),
        std::borrow::Cow::Owned(s) => s,
    }
}

fn encode_slow(url: &str, keep_escaped: bool) -> String {
    use std::fmt::Write;
    let mut encoded = String::with_capacity(url.len() + 16);
    for char in url.chars() {
        match char {
            'A'..='Z'
            | 'a'..='z'
            | '0'..='9'
            | '-'
            | '_'
            | '.'
            | '!'
            | '~'
            | '*'
            | '\''
            | '('
            | ')'
            | ';'
            | ','
            | '/'
            | '?'
            | ':'
            | '@'
            | '&'
            | '='
            | '+'
            | '$'
            | '#' => encoded.push(char),
            '%' if keep_escaped => encoded.push(char),
            _ => {
                for byte in encode_char(char, &mut [0; 4]) {
                    let _ = write!(encoded, "%{:02X}", byte);
                }
            }
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::text_encoding::encode_text;

    #[test]
    fn it_works() {
        assert_eq!(decode("?key=%E6%B5%8B%E8%AF%95"), "?key=测试");
        assert_eq!(decode("%3B%2C%2F%3F%3A%40%26%3D%2B%24"), ";,/?:@&=+$");
        assert_eq!(encode(" ", false), "%20");
        assert_eq!(encode("Hello World!", false), "Hello%20World!");
    }
    #[test]
    fn test_double_byte() {
        println!(
            "0xC3A4 == {:02X?} as {:02X?} as {:02X?} ",
            encode("ä", false),
            ('ä' as u32).to_be_bytes(),
            encode_text("ä")
        );
        assert_eq!(encode("\u{00E4}", false), "%C3%A4")
    }
}
