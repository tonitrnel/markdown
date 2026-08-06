use crate::ast::{self, MarkdownNode};
use crate::parser::Parser;
use crate::span::Span;

mod block_quote;
mod callout;
mod code;
mod footnote;
mod heading;
pub(crate) mod html;
mod list;
mod math;
mod table;
mod thematic_break;

pub enum BlockMatching {
    Unmatched = 0,
    MatchedContainer,
    MatchedLeaf,
}
pub enum BlockProcessing {
    Unprocessed = 0,
    Processed,
    Further,
}

pub struct BeforeCtx<'a, 'input> {
    pub container: usize,
    pub parser: &'a mut Parser<'input>,
    pub line: &'a mut Span<'input>,
}
pub struct ProcessCtx<'a, 'input> {
    pub id: usize,
    pub parser: &'a mut Parser<'input>,
    pub line: &'a mut Span<'input>,
}

pub trait BlockStrategy {
    /// 初始化容器
    ///
    /// 该函数将检查 Line 是否符合当前 Block 定义，如果符合则向 Parser Tree 上创建当前 Block
    ///
    /// 返回值:
    /// - `BlockMatching::Unmatched` 不匹配该 Block 定义
    /// - `BlockMatching::MatchedLeaf` 已匹配并且创建了 Block，该 Block 不支持嵌套其他 Block
    /// - `BlockMatching::MatchedContainer` 已匹配并且创建了 Block，该 Block 支持嵌套其他 Block，需要进一步拆分
    fn before(ctx: BeforeCtx) -> BlockMatching;

    /// 继续处理
    ///
    /// 该函数将为未关闭的 Block 进行处理
    ///
    /// 返回值：
    /// - `BlockProcessing::Unprocessed` 未处理，后续步骤应该退出当前容器
    /// - `BlockProcessing::Processed` 已处理，后续步骤也应该退出当前容器
    /// - `BlockProcessing::Further` 可以继续处理
    fn process(ctx: ProcessCtx) -> BlockProcessing;
    /// 节点即将关闭
    fn after(_id: usize, _parser: &mut Parser) {}
}

pub fn process<'input>(
    id: usize,
    parser: &mut Parser<'input>,
    line: &mut Span<'input>,
) -> BlockProcessing {
    let ctx = ProcessCtx { id, parser, line };
    match ctx.parser.tree[id].body {
        MarkdownNode::Document => BlockProcessing::Further,
        MarkdownNode::Heading(ast::heading::Heading::ATX(..)) => {
            ast::heading::ATXHeading::process(ctx)
        }
        MarkdownNode::Heading(ast::heading::Heading::SETEXT(..)) => {
            ast::heading::SetextHeading::process(ctx)
        }
        MarkdownNode::BlockQuote => ast::block_quote::BlockQuote::process(ctx),
        MarkdownNode::Code(ref c) if matches!(c.as_ref(), ast::code::Code::Fenced(..)) => {
            ast::code::FencedCode::process(ctx)
        }
        MarkdownNode::Code(ref c) if matches!(c.as_ref(), ast::code::Code::Indented(..)) => {
            ast::code::IndentedCode::process(ctx)
        }
        MarkdownNode::Html(..) => ast::html::Html::process(ctx),
        MarkdownNode::List(..) => ast::list::List::process(ctx),
        MarkdownNode::ListItem(..) => ast::list::ListItem::process(ctx),
        MarkdownNode::Table(..) => ast::table::Table::process(ctx),
        MarkdownNode::Callout(..) => ast::callout::Callout::process(ctx),
        MarkdownNode::Footnote(..) => ast::footnote::Footnote::process(ctx),
        MarkdownNode::Math(..) => ast::math::BlockMath::process(ctx),
        MarkdownNode::Paragraph => {
            if ctx.line.is_blank_to_end() {
                BlockProcessing::Unprocessed
            } else {
                BlockProcessing::Further
            }
        }
        _ => BlockProcessing::Unprocessed,
    }
}

pub fn after(id: usize, parser: &mut Parser, location: u32) {
    // For container blocks like List and ListItem, adjust end location
    // to not include the trailing newline of the last line
    let adjusted_location = match &parser.tree[id].body {
        MarkdownNode::List(..) | MarkdownNode::ListItem(..) => {
            // Use the end of the last child if available
            if let Some(last_child) = parser.tree.get_last_child(id) {
                parser.tree[last_child].span.end
            } else {
                location
            }
        }
        _ => location,
    };

    let node = &mut parser.tree[id];
    node.span.end = adjusted_location;

    match node.body {
        MarkdownNode::Heading(ast::heading::Heading::ATX(..)) => {
            ast::heading::ATXHeading::after(id, parser)
        }
        MarkdownNode::Heading(ast::heading::Heading::SETEXT(..)) => {
            ast::heading::SetextHeading::after(id, parser)
        }
        MarkdownNode::BlockQuote => ast::block_quote::BlockQuote::after(id, parser),
        MarkdownNode::Code(ref c) if matches!(c.as_ref(), ast::code::Code::Fenced(..)) => {
            ast::code::FencedCode::after(id, parser)
        }
        MarkdownNode::Code(ref c) if matches!(c.as_ref(), ast::code::Code::Indented(..)) => {
            ast::code::IndentedCode::after(id, parser)
        }
        MarkdownNode::List(..) => ast::list::List::after(id, parser),
        MarkdownNode::ListItem(..) => ast::list::ListItem::after(id, parser),
        MarkdownNode::Table(..) => ast::table::Table::after(id, parser),
        MarkdownNode::Callout(..) => ast::callout::Callout::after(id, parser),
        MarkdownNode::Footnote(..) => ast::footnote::Footnote::after(id, parser),
        MarkdownNode::Math(..) => ast::math::BlockMath::after(id, parser),
        _ => (),
    }
}
/// LineHead（v2C C1）：行首（缩进后）字节 → 候选 matcher 位集。
/// 位序 == `MATCHERS` 数组的既有优先级；省略的 matcher 对该首字节恒 Unmatched
/// （各 `before()` 的首字节门逐一核对，见 ticket 26 Proposal）。
/// 缩进行由调用方追加 IndentedCode（末位）。
static LINE_HEAD: [u16; 256] = {
    let mut t = [0u16; 256];
    t[b'>' as usize] = (1 << 0) | (1 << 1); // Callout, BlockQuote
    t[b'#' as usize] = 1 << 2; // ATXHeading
    t[b'`' as usize] = 1 << 3; // FencedCode
    t[b'~' as usize] = 1 << 3;
    t[b'<' as usize] = 1 << 4; // Html
    t[b'=' as usize] = 1 << 5; // SetextHeading
    t[b'-' as usize] = (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8);
    t[b'*' as usize] = (1 << 6) | (1 << 7); // ThematicBreak, ListItem
    t[b'_' as usize] = 1 << 6;
    t[b'+' as usize] = 1 << 7; // ListItem
    let mut d = b'0';
    while d <= b'9' {
        t[d as usize] = 1 << 7;
        d += 1;
    }
    t[b'|' as usize] = 1 << 8; // Table
    t[b':' as usize] = 1 << 8;
    t[b'[' as usize] = 1 << 9; // Footnote
    t[b'$' as usize] = 1 << 11; // BlockMath
    t
};

pub fn matcher<'input>(
    container: usize,
    parser: &mut Parser<'input>,
    line: &mut Span<'input>,
) -> BlockMatching {
    let snapshot = line.snapshot();
    let mut mask = line
        .get(line.indent_len())
        .map(|b| LINE_HEAD[b as usize])
        .unwrap_or(0);
    if line.is_indented() {
        mask |= 1 << 10; // IndentedCode（末位，保持原相对顺序）
    }
    // 展开的位测试保持直接调用（可内联）；顺序 == 既有 11-matcher 优先级
    macro_rules! try_matcher {
        ($bit:literal, $before:path) => {
            if mask & (1 << $bit) != 0 {
                line.resume(&snapshot);
                let ctx = BeforeCtx {
                    container,
                    parser,
                    line,
                };
                match $before(ctx) {
                    BlockMatching::Unmatched => {}
                    r => return r,
                }
            }
        };
    }
    try_matcher!(0, ast::callout::Callout::before);
    try_matcher!(1, ast::block_quote::BlockQuote::before);
    try_matcher!(2, ast::heading::ATXHeading::before);
    try_matcher!(3, ast::code::FencedCode::before);
    try_matcher!(4, ast::html::Html::before);
    try_matcher!(5, ast::heading::SetextHeading::before);
    try_matcher!(6, ast::thematic_break::ThematicBreak::before);
    try_matcher!(7, ast::list::ListItem::before);
    try_matcher!(8, ast::table::Table::before);
    try_matcher!(9, ast::footnote::Footnote::before);
    try_matcher!(10, ast::code::IndentedCode::before);
    try_matcher!(11, ast::math::BlockMath::before);
    line.resume(&snapshot);
    BlockMatching::Unmatched
}

pub(crate) fn reprocess<'input>(
    id: usize,
    parser: &mut Parser<'input>,
    line: &mut Span<'input>,
) -> bool {
    let snapshot = line.snapshot();
    let ctx = ProcessCtx { id, parser, line };
    let processed = match &ctx.parser.tree[ctx.id].body {
        MarkdownNode::Table(..) | MarkdownNode::TableBody => ast::table::Table::reprocess(ctx),
        _ => false,
    };
    if !processed {
        line.resume(&snapshot);
    }
    processed
}
