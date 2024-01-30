use crate::utils::text_encoding::decode_text;

fn decode_uri(uri: &str) -> String {
    let chars = uri.chars().collect::<Vec<_>>();
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
    decode_text(&bytes[..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", decode_uri("?key=%E6%B5%8B%E8%AF%95"));
        println!("{}", decode_uri("%3B%2C%2F%3F%3A%40%26%3D%2B%24"))
    }
}
