//! CJK 识别和自动处理 CJK 和 ASCII 的空格
//!
//! 参考 [chinese-copywriting-guidelines](https://github.com/sparanoid/chinese-copywriting-guidelines)
//!
//! 注: 本模块仅实现「空格」部分，对于「标点符号」、「名词」、「争议」部分本模块不予处理。
use std::borrow::Cow;

/// 判断字符是否为 CJK 字符（含标点、假名、全角符号等）
#[inline]
pub fn is_cjk(ch: char) -> bool {
    is_cjk_ideograph(ch) || is_cjk_punct_or_symbol(ch)
}

/// 判断字符是否为 CJK 表意文字或假名（用于空格插入判断）
#[inline]
fn is_cjk_ideograph(ch: char) -> bool {
    matches!(ch,
        '\u{4E00}'..='\u{9FFF}'     // CJK Unified Ideographs
        | '\u{3400}'..='\u{4DBF}'   // CJK Unified Ideographs Extension A
        | '\u{F900}'..='\u{FAFF}'   // CJK Compatibility Ideographs
        | '\u{20000}'..='\u{2A6DF}' // CJK Unified Ideographs Extension B
        | '\u{2A700}'..='\u{2B73F}' // CJK Unified Ideographs Extension C
        | '\u{2B740}'..='\u{2B81F}' // CJK Unified Ideographs Extension D
        | '\u{2B820}'..='\u{2CEAF}' // CJK Unified Ideographs Extension E
        | '\u{2CEB0}'..='\u{2EBEF}' // CJK Unified Ideographs Extension F
        | '\u{30000}'..='\u{3134F}' // CJK Unified Ideographs Extension G
        | '\u{3040}'..='\u{309F}'   // Hiragana
        | '\u{30A0}'..='\u{30FF}'   // Katakana
        | '\u{31F0}'..='\u{31FF}'   // Katakana Phonetic Extensions
    )
}

/// CJK 标点、符号、全角形式（不触发空格插入）
#[inline]
fn is_cjk_punct_or_symbol(ch: char) -> bool {
    matches!(ch,
        '\u{3000}'..='\u{303F}'   // CJK Symbols and Punctuation
        | '\u{FF00}'..='\u{FFEF}' // Halfwidth and Fullwidth Forms
        | '\u{FE30}'..='\u{FE4F}' // CJK Compatibility Forms
    )
}

#[inline]
fn is_ascii_alnum(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
}
/// 在 CJK 字符与 ASCII 字母数字之间插入空格。
/// 单次 O(n) 扫描，无需修改时返回 `Cow::Borrowed` 避免分配。
#[allow(dead_code)]
pub fn correct_cjk_spacing(text: &str) -> Cow<'_, str> {
    correct_cjk_spacing_with_nouns(text, (&[] as &[&str]).iter())
}
/// 在 CJK 字符与 ASCII 字母数字之间插入空格，跳过名词表中的专有名词。
/// `nouns` 中的词会被原样保留，不在其内部插入空格（如 "豆瓣FM"）。
pub fn correct_cjk_spacing_with_nouns<'a, I, S>(text: &'a str, nouns: I) -> Cow<'a, str>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    // 快速路径：收集名词在 text 中的所有出现区间 [start, end)
    let mut skip_ranges: Vec<(usize, usize)> = Vec::new();
    for noun in nouns {
        let noun = noun.as_ref();
        let mut search_from = 0;
        while let Some(pos) = text[search_from..].find(noun) {
            let abs_start = search_from + pos;
            let abs_end = abs_start + noun.len();
            skip_ranges.push((abs_start, abs_end));
            search_from = abs_end;
        }
    }
    skip_ranges.sort_unstable();

    let mut result: Option<String> = None;
    let mut chars = text.char_indices().peekable();
    let mut last_copied = 0;

    while let Some(&(_, ch)) = chars.peek() {
        chars.next();
        if let Some(&(next_i, next_ch)) = chars.peek() {
            let need_space = (is_cjk_ideograph(ch) && is_ascii_alnum(next_ch))
                || (is_ascii_alnum(ch) && is_cjk_ideograph(next_ch));

            if need_space && !in_skip_range(&skip_ranges, next_i) {
                let buf = result.get_or_insert_with(|| String::with_capacity(text.len() + 16));
                buf.push_str(&text[last_copied..next_i]);
                buf.push(' ');
                last_copied = next_i;
            }
        }
    }

    match result {
        Some(mut buf) => {
            buf.push_str(&text[last_copied..]);
            Cow::Owned(buf)
        }
        None => Cow::Borrowed(text),
    }
}

/// 检查字节偏移 `pos` 处的边界是否被某个跳过区间覆盖。
/// 空格插入发生在 pos 前一个字符和 pos 处字符之间。
/// 如果 pos 落在 [start, end] 范围内，说明边界的某一侧属于专有名词，应跳过。
#[inline]
fn in_skip_range(ranges: &[(usize, usize)], pos: usize) -> bool {
    ranges
        .iter()
        .any(|&(start, end)| pos >= start && pos <= end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Property 3: CJK 空格插入正确性
    // **Validates: Requirements 3.2, 3.3**
    proptest! {
        #[test]
        fn prop_cjk_spacing_correctness(s in "[a-zA-Z0-9\u{4E00}-\u{9FFF} ]{1,60}") {
            // Feature: performance-optimization, Property 3: CJK 空格插入正确性
            let result = correct_cjk_spacing(&s);
            let chars: Vec<char> = result.chars().collect();
            for i in 0..chars.len().saturating_sub(1) {
                let a = chars[i];
                let b = chars[i + 1];
                if is_cjk_ideograph(a) && is_ascii_alnum(b) {
                    panic!("CJK char '{}' directly adjacent to ASCII '{}' at position {}", a, b, i);
                }
                if is_ascii_alnum(a) && is_cjk_ideograph(b) {
                    panic!("ASCII '{}' directly adjacent to CJK char '{}' at position {}", a, b, i);
                }
            }
        }
    }

    // Property 4: CJK 校正幂等性
    // **Validates: Requirements 3.4, 3.5, 3.6**
    proptest! {
        #[test]
        fn prop_cjk_correction_idempotent(s in "\\PC{1,80}") {
            // Feature: performance-optimization, Property 4: CJK 校正幂等性
            let once = correct_cjk_spacing(&s);
            let twice = correct_cjk_spacing(&once);
            prop_assert_eq!(&*once, &*twice, "Idempotency violated: applying correction twice differs from once");
        }
    }

    // ---- 基于 chinese-copywriting-guidelines 的示例测试 ----
    // 参考: https://github.com/sparanoid/chinese-copywriting-guidelines

    #[test]
    fn cjk_english_spacing() {
        // 中英文之间需要增加空格
        assert_eq!(
            correct_cjk_spacing("在LeanCloud上，數據儲存是圍繞AVObject進行的。").as_ref(),
            "在 LeanCloud 上，數據儲存是圍繞 AVObject 進行的。"
        );
    }

    #[test]
    fn cjk_number_spacing() {
        // 中文与数字之间需要增加空格
        assert_eq!(
            correct_cjk_spacing("今天出去買菜花了5000元。").as_ref(),
            "今天出去買菜花了 5000 元。"
        );
    }

    #[test]
    fn already_spaced() {
        // 已有空格时不重复插入
        let input = "在 LeanCloud 上，數據儲存是圍繞 AVObject 進行的。";
        assert_eq!(correct_cjk_spacing(input).as_ref(), input);
    }

    #[test]
    fn no_cjk_passthrough() {
        // 纯 ASCII 不修改
        let input = "Hello World 123";
        assert!(matches!(correct_cjk_spacing(input), Cow::Borrowed(_)));
        assert_eq!(correct_cjk_spacing(input).as_ref(), input);
    }

    #[test]
    fn pure_cjk_passthrough() {
        // 纯中文不修改
        let input = "你好世界";
        assert!(matches!(correct_cjk_spacing(input), Cow::Borrowed(_)));
    }

    #[test]
    fn fullwidth_punct_no_space() {
        // 全形标点与其他字符之间不加空格
        // correct_cjk_spacing 只处理 CJK-ASCII alnum 边界，标点不受影响
        assert_eq!(
            correct_cjk_spacing("剛剛買了一部iPhone，好開心").as_ref(),
            "剛剛買了一部 iPhone，好開心"
        );
    }

    #[test]
    fn complete_sentence() {
        // 完整的正确用法示例
        assert_eq!(
            correct_cjk_spacing(
                "每個AVObject都包含了與JSON兼容的key-value對應的數據。數據是schema-free的"
            )
            .as_ref(),
            "每個 AVObject 都包含了與 JSON 兼容的 key-value 對應的數據。數據是 schema-free 的"
        );
    }

    #[test]
    fn noun_list_skip() {
        // 专有名词跳过：「豆瓣FM」按官方格式保留
        let nouns = &["豆瓣FM"];
        assert_eq!(
            correct_cjk_spacing_with_nouns("我最愛的產品是簡書和豆瓣FM，你呢？", nouns.iter())
                .as_ref(),
            "我最愛的產品是簡書和豆瓣FM，你呢？"
        );
    }

    #[test]
    fn noun_list_multiple() {
        // 多个专有名词，名词边界处也不插入空格
        let nouns = &["豆瓣FM", "QQ音乐"];
        assert_eq!(
            correct_cjk_spacing_with_nouns("我用豆瓣FM和QQ音乐聽歌", nouns.iter()).as_ref(),
            "我用豆瓣FM和QQ音乐聽歌"
        );
    }

    #[test]
    fn noun_list_empty_still_corrects() {
        // 空名词表 = 正常校正
        assert_eq!(
            correct_cjk_spacing_with_nouns("使用GitHub登錄", (&[] as &[&str]).iter()).as_ref(),
            "使用 GitHub 登錄"
        );
    }

    #[test]
    fn noun_list_no_match() {
        // 名词表中的词不在文本中，正常校正
        let nouns = &["豆瓣FM"];
        assert_eq!(
            correct_cjk_spacing_with_nouns("使用GitHub登錄", nouns.iter()).as_ref(),
            "使用 GitHub 登錄"
        );
    }

    #[test]
    fn mixed_number_and_unit() {
        // 数字与单位之间增加空格
        assert_eq!(
            correct_cjk_spacing("我家的光纖入屋寬頻有10Gbps").as_ref(),
            "我家的光纖入屋寬頻有 10Gbps"
        );
    }

    #[test]
    fn empty_string() {
        assert!(matches!(correct_cjk_spacing(""), Cow::Borrowed(_)));
        assert_eq!(correct_cjk_spacing("").as_ref(), "");
    }
}
