use crate::ast::{self, MarkdownNode};
use crate::line::Line;
use crate::parser::Parser;
use crate::tokenizer::Location;

mod block_quote;
mod code;
mod heading;
mod html;
mod list;
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
    pub line: &'a mut Line<'input>,
}
pub struct ProcessCtx<'a, 'input> {
    pub id: usize,
    pub parser: &'a mut Parser<'input>,
    pub line: &'a mut Line<'input>,
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
    fn after(_id: usize, _parser: &mut Parser) {}
}

pub fn process<'input>(
    id: usize,
    parser: &mut Parser<'input>,
    line: &mut Line<'input>,
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
        MarkdownNode::BlockQuote(ast::block_quote::BlockQuote {}) => {
            ast::block_quote::BlockQuote::process(ctx)
        }
        MarkdownNode::Code(ast::code::Code::Fenced(..)) => ast::code::FencedCode::process(ctx),
        MarkdownNode::Code(ast::code::Code::Indented(..)) => ast::code::IndentedCode::process(ctx),
        MarkdownNode::Html(..) => ast::html::Html::process(ctx),
        MarkdownNode::List(..) => ast::list::List::process(ctx),
        MarkdownNode::ListItem(..) => ast::list::ListItem::process(ctx),
        _ => BlockProcessing::Unprocessed,
    }
}

pub fn after(id: usize, parser: &mut Parser, location: Location) {
    let node = &mut parser.tree[id];
    node.end = location;
    match node.body {
        MarkdownNode::Heading(ast::heading::Heading::ATX(..)) => {
            ast::heading::ATXHeading::after(id, parser)
        }
        MarkdownNode::Heading(ast::heading::Heading::SETEXT(..)) => {
            ast::heading::SetextHeading::after(id, parser)
        }
        MarkdownNode::BlockQuote(ast::block_quote::BlockQuote {}) => {
            ast::block_quote::BlockQuote::after(id, parser)
        }
        MarkdownNode::Code(ast::code::Code::Fenced(..)) => ast::code::FencedCode::after(id, parser),
        MarkdownNode::Code(ast::code::Code::Indented(..)) => {
            ast::code::IndentedCode::after(id, parser)
        }
        MarkdownNode::List(..) => ast::list::List::after(id, parser),
        _ => (),
    }
}

pub fn matcher<'input>(
    container: usize,
    parser: &mut Parser<'input>,
    line: &mut Line<'input>,
) -> BlockMatching {
    let matchers = [
        ast::block_quote::BlockQuote::before,
        ast::heading::ATXHeading::before,
        ast::code::FencedCode::before,
        ast::html::Html::before,
        ast::heading::SetextHeading::before,
        ast::thematic_break::ThematicBreak::before,
        ast::list::ListItem::before,
        ast::code::IndentedCode::before,
    ];
    let snapshot = line.snapshot();
    for matcher in matchers {
        line.resume(&snapshot);
        let ctx = BeforeCtx {
            container,
            parser,
            line,
        };
        match matcher(ctx) {
            BlockMatching::Unmatched => continue,
            r => return r,
        }
    }
    BlockMatching::Unmatched
}
