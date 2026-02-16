use crate::inlines::{bracket::BracketChain, delimiter::DelimiterChain};
use crate::parser::Location;
use crate::parser::Parser;
use crate::span::{MergedSpan, Span};

mod bracket;
mod code;
mod comment;
mod delimiter;
mod emoji;
mod entity;
mod footnote;
mod html;
mod link;
mod link_reference;
mod math;
mod newline;
mod tag;
#[cfg(test)]
mod tests;
mod text;

pub(crate) use footnote::process_footnote_list;
pub(crate) use link_reference::is_link_reference_line;
pub(crate) use link_reference::process_link_reference;
pub(crate) use link_reference::process_setext_heading_link_reference;

struct ProcessCtx<'a, 'input> {
    id: usize,
    parser: &'a mut Parser<'input>,
    line: &'a mut MergedSpan<'input>,
    brackets: Option<BracketChain>,
    delimiters: Option<DelimiterChain>,
}

/// 处理 inline 元素。使用 MergedSpan 将多个 Span 合并为统一视图，
/// 在 Span 之间自动插入换行符，避免包含中间行的前缀字符。
pub(super) fn process<'input>(id: usize, parser: &mut Parser<'input>, spans: Vec<Span<'input>>) {
    if spans.is_empty() {
        return;
    }

    // 规范化每个 Span：将 start 调整为 cursor 位置（去掉已跳过的缩进）
    let normalized: Vec<Span<'input>> = spans
        .into_iter()
        .map(|mut s| {
            s.normalize_start();
            s
        })
        .collect();

    // 使用 MergedSpan 提供统一的字节视图，避免把中间行前缀（如 blockquote 的 `>`）混入 inline 内容
    let mut line = MergedSpan::new(normalized);

    let mut ctx = ProcessCtx {
        id,
        parser,
        line: &mut line,
        brackets: None,
        delimiters: None,
    };

    // 选择查找表
    let special_table: &[bool; 256] = if ctx.parser.options.obsidian_flavored {
        &SPECIAL_BYTES_OFM
    } else if ctx.parser.options.github_flavored && ctx.parser.options.gfm_extended_autolink {
        &SPECIAL_BYTES_GFM_EXTENDED_AUTOLINK
    } else if ctx.parser.options.github_flavored {
        &SPECIAL_BYTES_GFM
    } else {
        &SPECIAL_BYTES_DEFAULT
    };

    // 累积连续文本的字节偏移范围，避免逐字符 to_string() 堆分配
    let mut text_acc: Option<TextAccumulator> = None;

    while let Some(byte) = ctx.line.peek() {
        if !special_table[byte as usize] {
            // 非特殊字节：直接批量累积，不需要 snapshot/resume
            accumulate_run(&mut text_acc, &mut ctx, special_table);
            continue;
        }

        // 特殊字符：先 flush 累积的文本，然后尝试匹配
        flush_text_acc(&mut text_acc, &mut ctx);
        let snapshot = ctx.line.snapshot();

        let handled = match byte {
            // Hard break, Soft break (换行符)
            b'\n' | b'\r' => newline::process(&mut ctx),
            // Backslash
            b'\\' => newline::process_backslash(&mut ctx),
            // Code
            b'`' => code::process(&mut ctx),
            // Emphasis, Strong emphasis
            b'*' | b'_' => delimiter::before(&mut ctx, false, false),
            // Smart punctuation quotes
            b'\'' | b'"' if ctx.parser.options.smart_punctuation => {
                delimiter::before(&mut ctx, false, false)
            }
            // Strikethrough(GFM)
            b'~' if ctx.parser.options.github_flavored || ctx.parser.options.obsidian_flavored => {
                delimiter::before(&mut ctx, true, false)
            }
            // Highlight(OFM)
            b'=' if ctx.parser.options.obsidian_flavored => {
                delimiter::before(&mut ctx, false, true)
            }
            // Link or Wikilink
            b'[' => {
                if let Some(current_span) = ctx.line.current_span() {
                    if current_span.get(1) == Some(b'[') && ctx.parser.options.obsidian_flavored {
                        link::process_wikilink(&mut ctx)
                    } else {
                        bracket::before(&mut ctx, false)
                    }
                } else {
                    bracket::before(&mut ctx, false)
                }
            }
            // Image, Embed(OFM)
            b'!' => {
                if let Some(current_span) = ctx.line.current_span() {
                    match current_span.get(1) {
                        Some(b'[') => {
                            if current_span.get(2) == Some(b'[')
                                && ctx.parser.options.obsidian_flavored
                            {
                                link::process_embed(&mut ctx)
                            } else {
                                bracket::before(&mut ctx, true)
                            }
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            // Close bracket
            b']' => bracket::process(&mut ctx),
            // Entity
            b'&' => entity::process(&mut ctx),
            // AutoLinks, Raw HTML
            b'<' => 'multi: {
                if link::process_autolink(&mut ctx) {
                    break 'multi true;
                }
                ctx.line.resume(&snapshot);
                if html::process(&mut ctx) {
                    break 'multi true;
                }
                false
            }
            // GFM extended autolink: http(s)://... and www....
            b'h' | b'H' | b'w' | b'W'
                if ctx.parser.options.github_flavored
                    && ctx.parser.options.gfm_extended_autolink =>
            {
                link::process_gfm_autolink(&mut ctx)
            }
            b'{' if ctx.parser.options.mdx_component => html::process(&mut ctx),
            // Math ($)
            0x24 if !ctx.parser.options.default_flavored => {
                if let Some(current_span) = ctx.line.current_span() {
                    let is_block = current_span.validate(1, 0x24);
                    math::process(&mut ctx, is_block)
                } else {
                    false
                }
            }
            // Block id(OFM) (^)
            b'^' if ctx.parser.options.obsidian_flavored => link::process_block_id(&mut ctx),
            // Emoji (:)
            b':' if !ctx.parser.options.default_flavored => emoji::process(&mut ctx),
            // Tag (#)
            b'#' if ctx.parser.options.obsidian_flavored => tag::process(&mut ctx),
            // Comment (%%)
            b'%' if ctx.parser.options.obsidian_flavored => {
                if let Some(current_span) = ctx.line.current_span() {
                    if current_span.get(1) == Some(b'%') {
                        comment::process(&mut ctx)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        };
        if !handled {
            // 未匹配，恢复快照并累积当前字符
            ctx.line.resume(&snapshot);
            accumulate_run(&mut text_acc, &mut ctx, special_table);
        }
    }
    // 处理结束，刷新剩余累积文本
    flush_text_acc(&mut text_acc, &mut ctx);

    // 最终处理 delimiter 和 text
    delimiter::process_final(id, ctx.parser, &mut ctx.brackets, &mut ctx.delimiters);
    text::process_final(id, ctx.parser);
}

/// 文本累积器：零分配快速路径 + 回退 String 路径
///
/// 大多数文本在同一个 Span 内连续，用 source 切片引用即可（零分配）。
/// 只有跨 Span 或 Span 边界回退时才 fallback 到 String。
enum TextAccumulator<'a> {
    /// 零分配：同一 Span 内的连续文本，直接引用 source 切片
    Slice {
        /// source buffer 指针，用于判断是否同一 Span
        source_ptr: *const u8,
        slice: &'a str,
        /// 切片在 source 中的结束偏移，用于判断是否可以原地扩展
        range_end: usize,
        start_location: Location,
    },
    /// 回退路径：跨 Span 或需要拼接时用 String
    Owned {
        text: String,
        start_location: Location,
    },
}

/// 256 字节查找表：OFM 模式下的特殊字节（编译期常量）
static SPECIAL_BYTES_OFM: [bool; 256] = {
    let mut table = [false; 256];
    // 基础特殊字符
    table[b'\n' as usize] = true;
    table[b'\r' as usize] = true;
    table[b'\\' as usize] = true;
    table[b'`' as usize] = true;
    table[b'*' as usize] = true;
    table[b'_' as usize] = true;
    table[b'[' as usize] = true;
    table[b'!' as usize] = true;
    table[b']' as usize] = true;
    table[b'&' as usize] = true;
    table[b'<' as usize] = true;
    table[b'{' as usize] = true;
    // GFM + OFM
    table[b'~' as usize] = true;
    // OFM
    table[b'=' as usize] = true;
    table[b'^' as usize] = true;
    table[b'#' as usize] = true;
    table[b'%' as usize] = true;
    // non-default: math, emoji
    table[0x24] = true; // $
    table[b':' as usize] = true;
    // Smart punctuation quotes
    table[b'\'' as usize] = true;
    table[b'"' as usize] = true;
    table
};

/// 256 字节查找表：默认模式下的特殊字节
static SPECIAL_BYTES_DEFAULT: [bool; 256] = {
    let mut table = [false; 256];
    table[b'\n' as usize] = true;
    table[b'\r' as usize] = true;
    table[b'\\' as usize] = true;
    table[b'`' as usize] = true;
    table[b'*' as usize] = true;
    table[b'_' as usize] = true;
    table[b'[' as usize] = true;
    table[b'!' as usize] = true;
    table[b']' as usize] = true;
    table[b'&' as usize] = true;
    table[b'<' as usize] = true;
    table[b'{' as usize] = true;
    // Smart punctuation quotes
    table[b'\'' as usize] = true;
    table[b'"' as usize] = true;
    table
};

/// 256 字节查找表：GFM 模式下的特殊字节
static SPECIAL_BYTES_GFM: [bool; 256] = {
    let mut table = [false; 256];
    table[b'\n' as usize] = true;
    table[b'\r' as usize] = true;
    table[b'\\' as usize] = true;
    table[b'`' as usize] = true;
    table[b'*' as usize] = true;
    table[b'_' as usize] = true;
    table[b'[' as usize] = true;
    table[b'!' as usize] = true;
    table[b']' as usize] = true;
    table[b'&' as usize] = true;
    table[b'<' as usize] = true;
    table[b'{' as usize] = true;
    table[b'~' as usize] = true;
    // non-default
    table[0x24] = true;
    table[b':' as usize] = true;
    // Smart punctuation quotes
    table[b'\'' as usize] = true;
    table[b'"' as usize] = true;
    table
};

/// 256 字节查找表：GFM + extended autolink
static SPECIAL_BYTES_GFM_EXTENDED_AUTOLINK: [bool; 256] = {
    let mut table = SPECIAL_BYTES_GFM;
    table[b'h' as usize] = true;
    table[b'H' as usize] = true;
    table[b'w' as usize] = true;
    table[b'W' as usize] = true;
    table
};

/// 批量累积连续非特殊字节。
/// 快速路径：同一 Span 内直接用 source 切片引用，零分配。
/// 优化点：同一 Span 内连续调用时，Slice 可原地扩展而不退化为 Owned。
fn accumulate_run<'input>(
    text_acc: &mut Option<TextAccumulator<'input>>,
    ctx: &mut ProcessCtx<'_, 'input>,
    special_table: &[bool; 256],
) {
    // 尝试在当前 Span 内批量扫描
    if let Some(span) = ctx.line.current_span() {
        let source = span.source_slice();
        let start = span.cursor();
        let end = span.end();
        let mut pos = start;

        // 查表扫描到下一个特殊字节或 Span 末尾
        // 注意：所有特殊字节都 < 0x80，所以 UTF-8 多字节序列的首字节（>= 0x80）
        // 一定不是特殊字节，可以直接跳过整个字符
        // SAFETY: pos < end <= source.len()，内循环用 get_unchecked 消除边界检查
        while pos < end {
            let b = unsafe { *source.get_unchecked(pos) };
            if b < 0x80 {
                if special_table[b as usize] {
                    break;
                }
                pos += 1;
            } else {
                // UTF-8 多字节字符：首字节一定不是特殊字节，直接跳过
                let step = if b < 0xE0 {
                    2
                } else if b < 0xF0 {
                    3
                } else {
                    4
                };
                pos += step;
                // 多字节字符可能跨越 Span 末尾
                if pos > end {
                    pos = end;
                    break;
                }
            }
        }

        if pos > start {
            let advance_count = pos - start;

            match text_acc {
                Some(acc) => {
                    push_span_chunk(acc, source, start, pos);
                }
                None => {
                    let chunk = unsafe { std::str::from_utf8_unchecked(&source[start..pos]) };
                    let start_loc = ctx.line.start_location();
                    *text_acc = Some(TextAccumulator::Slice {
                        source_ptr: source.as_ptr(),
                        slice: chunk,
                        range_end: pos,
                        start_location: start_loc,
                    });
                }
            }
            ctx.line.skip(advance_count);
            return;
        }
    }

    // 回退路径：Span 边界处理（换行符等）
    fallback_accumulate(text_acc, ctx);
}

/// 将同一 Span 内的新 chunk 追加到已有累积器。
/// 如果累积器是 Slice 且新 chunk 紧邻其后（同一 source buffer），直接原地扩展，零分配。
#[inline]
fn push_span_chunk<'input>(
    acc: &mut TextAccumulator<'input>,
    source: &'input [u8],
    start: usize,
    pos: usize,
) {
    match acc {
        TextAccumulator::Slice {
            source_ptr,
            slice,
            range_end,
            start_location,
        } => {
            // 同一 source buffer 且紧邻：原地扩展 slice，零分配
            if *source_ptr == source.as_ptr() && *range_end == start {
                let slice_start = *range_end - slice.len();
                *slice = unsafe { std::str::from_utf8_unchecked(&source[slice_start..pos]) };
                *range_end = pos;
            } else {
                // 跨 Span：Slice → Owned
                let chunk = unsafe { std::str::from_utf8_unchecked(&source[start..pos]) };
                let mut text = String::with_capacity(slice.len() + chunk.len());
                text.push_str(slice);
                text.push_str(chunk);
                *acc = TextAccumulator::Owned {
                    text,
                    start_location: *start_location,
                };
            }
        }
        TextAccumulator::Owned { text, .. } => {
            let chunk = unsafe { std::str::from_utf8_unchecked(&source[start..pos]) };
            text.push_str(chunk);
        }
    }
}

/// 回退路径：处理 Span 边界字符（如换行符等跨 Span 的字节）
#[inline]
fn fallback_accumulate<'input>(
    text_acc: &mut Option<TextAccumulator<'input>>,
    ctx: &mut ProcessCtx<'_, 'input>,
) {
    let pre_loc = ctx.line.start_location();
    if let Some(byte) = ctx.line.next_byte() {
        let char_len = if byte < 0x80 {
            1
        } else if byte < 0xE0 {
            2
        } else if byte < 0xF0 {
            3
        } else {
            4
        };
        let mut char_buf = [0u8; 4];
        char_buf[0] = byte;
        for i in 1..char_len {
            if let Some(b) = ctx.line.next_byte() {
                char_buf[i] = b;
            }
        }
        let ch = unsafe { std::str::from_utf8_unchecked(&char_buf[..char_len]) };
        match text_acc {
            Some(acc) => match acc {
                TextAccumulator::Slice {
                    slice,
                    start_location,
                    ..
                } => {
                    let mut text = String::with_capacity(slice.len() + char_len);
                    text.push_str(slice);
                    text.push_str(ch);
                    *acc = TextAccumulator::Owned {
                        text,
                        start_location: *start_location,
                    };
                }
                TextAccumulator::Owned { text, .. } => {
                    text.push_str(ch);
                }
            },
            None => {
                let mut text = String::with_capacity(64);
                text.push_str(ch);
                *text_acc = Some(TextAccumulator::Owned {
                    text,
                    start_location: pre_loc,
                });
            }
        }
    }
}

/// 将累积的文本一次性创建为 Text 节点
#[inline]
fn flush_text_acc(text_acc: &mut Option<TextAccumulator>, ctx: &mut ProcessCtx) {
    let Some(acc) = text_acc.take() else {
        return;
    };

    match acc {
        TextAccumulator::Slice {
            slice,
            start_location,
            ..
        } if !slice.is_empty() => {
            let end_loc = ctx.line.start_location();
            ctx.parser
                .append_text_to(ctx.id, slice, (start_location, end_loc));
        }
        TextAccumulator::Owned {
            text,
            start_location,
        } if !text.is_empty() => {
            let end_loc = ctx.line.start_location();
            ctx.parser
                .append_text_to_owned(ctx.id, text, (start_location, end_loc));
        }
        _ => {}
    }
}
