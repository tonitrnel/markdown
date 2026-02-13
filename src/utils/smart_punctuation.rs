use std::borrow::Cow;

/// 将 ASCII 标点转换为排版标点（Smart Punctuation）。
///
/// 转换规则（基于 CommonMark smart_punct 扩展）：
/// - `---` → `—`（em dash）
/// - `--` → `–`（en dash）
/// - `...` → `…`（ellipsis）
///
/// 引号转换（`"` → `""`，`'` → `''`）在 inline delimiter 阶段处理，
/// 此函数仅处理 dash 和 ellipsis。
pub fn smart_punctuation(text: &str) -> Cow<'_, str> {
    // 快速检查：无需处理则直接返回
    if !text.contains("--") && !text.contains("...") {
        return Cow::Borrowed(text);
    }

    let mut result = String::with_capacity(text.len());
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        match bytes[i] {
            b'-' => {
                // 计算连续 hyphen 数量
                let start = i;
                while i < len && bytes[i] == b'-' {
                    i += 1;
                }
                let count = i - start;
                emit_dashes(&mut result, count);
            }
            b'.' if i + 2 < len && bytes[i + 1] == b'.' && bytes[i + 2] == b'.' => {
                result.push('\u{2026}'); // …
                i += 3;
            }
            _ => {
                // 安全推进：按 UTF-8 字符边界前进
                let ch_start = i;
                i += 1;
                while i < len && !is_utf8_char_boundary(bytes[i]) {
                    i += 1;
                }
                result.push_str(&text[ch_start..i]);
            }
        }
    }

    Cow::Owned(result)
}

/// 将 n 个连续 hyphen 转换为 em dash 和 en dash 的组合。
///
/// 规则：尽量使用同类 dash。当必须混合时，em dash 在前，en dash 尽量少。
fn emit_dashes(out: &mut String, count: usize) {
    if count == 1 {
        out.push('-');
        return;
    }
    // em dash (---) 优先，剩余用 en dash (--)
    // 特殊情况：count % 3 == 0 → 全 em dash
    //           count % 3 == 2 → (count/3) em + 1 en
    //           count % 3 == 1 → (count/3 - 1) em + 2 en
    let (em, en) = match count % 3 {
        0 => (count / 3, 0),
        1 => {
            if count >= 4 {
                (count / 3 - 1, 2)
            } else {
                // count == 1 已处理
                (0, count / 2)
            }
        }
        2 => (count / 3, 1),
        _ => unreachable!(),
    };
    for _ in 0..em {
        out.push('\u{2014}'); // —
    }
    for _ in 0..en {
        out.push('\u{2013}'); // –
    }
}

#[inline]
fn is_utf8_char_boundary(b: u8) -> bool {
    // 非延续字节（不是 10xxxxxx）
    (b as i8) >= -0x40
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn em_dash() {
        assert_eq!(smart_punctuation("em---em").as_ref(), "em\u{2014}em");
    }

    #[test]
    fn en_dash() {
        assert_eq!(smart_punctuation("en--en").as_ref(), "en\u{2013}en");
    }

    #[test]
    fn ellipsis() {
        assert_eq!(
            smart_punctuation("Ellipses...and...and....").as_ref(),
            "Ellipses\u{2026}and\u{2026}and\u{2026}."
        );
    }

    #[test]
    fn mixed_dashes() {
        // 基于 CommonMark smart_punct.txt 的测试用例
        assert_eq!(smart_punctuation("one-").as_ref(), "one-");
        assert_eq!(smart_punctuation("two--").as_ref(), "two\u{2013}");
        assert_eq!(smart_punctuation("three---").as_ref(), "three\u{2014}");
        assert_eq!(
            smart_punctuation("four----").as_ref(),
            "four\u{2013}\u{2013}"
        );
        assert_eq!(
            smart_punctuation("five-----").as_ref(),
            "five\u{2014}\u{2013}"
        );
        assert_eq!(
            smart_punctuation("six------").as_ref(),
            "six\u{2014}\u{2014}"
        );
        // 7 hyphens = 1 em + 2 en (3 + 4 = 7)
        assert_eq!(
            smart_punctuation("seven-------").as_ref(),
            "seven\u{2014}\u{2013}\u{2013}"
        );
    }

    #[test]
    fn no_change() {
        let input = "Hello world";
        assert!(matches!(smart_punctuation(input), Cow::Borrowed(_)));
    }

    #[test]
    fn single_hyphen_preserved() {
        assert_eq!(smart_punctuation("well-known").as_ref(), "well-known");
    }

    #[test]
    fn cjk_with_dashes() {
        assert_eq!(
            smart_punctuation("你好---世界").as_ref(),
            "你好\u{2014}世界"
        );
    }
}
