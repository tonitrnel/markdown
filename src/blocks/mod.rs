pub use block_quote::*;
pub use code::*;
pub use heading::*;
pub use html::*;
pub use list::*;
pub use thematic_break::*;

use crate::ast::{self, MarkdownNode};
use crate::line::Line;
use crate::parser::Parser;

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

pub trait BlockStrategy {
    /// 初始化容器
    ///
    /// 该函数将检查 Line 是否符合当前 Block 定义，如果符合则向 Parser Tree 上创建当前 Block
    ///
    /// 返回值:
    /// - `BlockMatching::Unmatched` 不匹配该 Block 定义
    /// - `BlockMatching::MatchedLeaf` 已匹配并且创建了 Block，该 Block 不支持嵌套其他 Block
    /// - `BlockMatching::MatchedContainer` 已匹配并且创建了 Block，该 Block 支持嵌套其他 Block，需要进一步拆分
    fn before<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockMatching;

    /// 继续处理
    ///
    /// 该函数将为未关闭的 Block 进行处理
    ///
    /// 返回值：
    /// - `BlockProcessing::Unprocessed` 未处理，后续步骤应该退出当前容器
    /// - `BlockProcessing::Processed` 已处理，后续步骤也应该退出当前容器
    /// - `BlockProcessing::Further` 可以继续处理
    fn process<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockProcessing;
    fn after(_id: usize, _parser: &mut Parser) {}
}

pub fn process<'input>(
    id: usize,
    parser: &mut Parser<'input>,
    line: &mut Line<'input>,
) -> BlockProcessing {
    let node = &parser.tree[id].body;
    match node {
        MarkdownNode::Document => BlockProcessing::Further,
        MarkdownNode::Heading(ast::heading::Heading {
            variant: ast::heading::HeadingVariant::ATX,
            ..
        }) => ATXHeading::process(parser, line),
        MarkdownNode::Heading(ast::heading::Heading {
            variant: ast::heading::HeadingVariant::SETEXT,
            ..
        }) => SetextHeading::process(parser, line),
        MarkdownNode::BlockQuote => BlockQuote::process(parser, line),
        MarkdownNode::Code(ast::code::Code {
            variant: ast::code::CodeVariant::Fenced,
            ..
        }) => FencedCode::process(parser, line),
        MarkdownNode::Code(ast::code::Code {
            variant: ast::code::CodeVariant::Indented,
            ..
        }) => IndentedCode::process(parser, line),
        _ => BlockProcessing::Unprocessed,
    }
}

pub fn after(id: usize, parser: &mut Parser) {
    let node = &parser.tree[id];
    match node.body {
        MarkdownNode::Heading(ast::heading::Heading {
            variant: ast::heading::HeadingVariant::ATX,
            ..
        }) => ATXHeading::after(id, parser),
        MarkdownNode::Heading(ast::heading::Heading {
            variant: ast::heading::HeadingVariant::SETEXT,
            ..
        }) => SetextHeading::after(id, parser),
        MarkdownNode::BlockQuote => BlockQuote::after(id, parser),
        MarkdownNode::Code(ast::code::Code {
            variant: ast::code::CodeVariant::Fenced,
            ..
        }) => FencedCode::after(id, parser),
        MarkdownNode::Code(ast::code::Code {
            variant: ast::code::CodeVariant::Indented,
            ..
        }) => IndentedCode::after(id, parser),
        _ => (),
    }
}

pub fn matcher<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockMatching {
    let matchers = [
        BlockQuote::before,
        ATXHeading::before,
        FencedCode::before,
        HTML::before,
        SetextHeading::before,
        ThematicBreak::before,
        Item::before,
        IndentedCode::before,
    ];
    let snapshot = line.snapshot();
    for matcher in matchers {
        line.resume(&snapshot);
        match matcher(parser, line) {
            BlockMatching::Unmatched => continue,
            r => return r,
        }
    }
    BlockMatching::Unmatched
}
