use crate::ast::{table, MarkdownNode};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};
use crate::line::Line;
use crate::tokenizer::{Location, Token};
use std::ops::Range;

enum State {
    Initial,
    ColStart,
    Col,
    ColEnd,
    Invalid,
}
type Row<'input> = ((Location, Location), Vec<Line<'input>>);
type ColDefs = Vec<(Range<usize>, table::Alignment)>;

impl From<(bool, bool)> for table::Alignment {
    fn from(value: (bool, bool)) -> Self {
        match value {
            (false, false) | (true, false) => Self::Left,
            (false, true) => Self::Right,
            (true, true) => Self::Center,
        }
    }
}
impl table::Table {
    fn scan_columns(line: &mut Line) -> Option<ColDefs> {
        let mut col_defs: ColDefs = Vec::new();
        let mut range = Range { start: 0, end: 0 };
        let mut flags = (false, false);
        let mut state = State::Initial;
        while let Some(token) = line.peek() {
            state = match state {
                State::Initial => match token {
                    Token::Pipe => {
                        line.next();
                        range.start = line.start_offset;
                        State::ColStart
                    }
                    Token::Hyphen | Token::Whitespace(..) | Token::Colon => {
                        range.start = line.start_offset;
                        State::ColStart
                    }
                    _ => State::Invalid,
                },
                State::ColStart => match token {
                    Token::Whitespace(..) => {
                        line.next();
                        State::ColStart
                    }
                    Token::Colon => {
                        flags.0 = true;
                        line.next();
                        State::ColStart
                    }
                    Token::Hyphen => {
                        line.next();
                        State::Col
                    }
                    _ => State::Invalid,
                },
                State::Col => match token {
                    Token::Hyphen => {
                        flags.1 = false;
                        line.next();
                        State::Col
                    }
                    Token::Whitespace(..) => {
                        line.next();
                        State::Col
                    }
                    Token::Colon => {
                        flags.1 = true;
                        line.next();
                        State::Col
                    }
                    Token::Pipe => State::ColEnd,
                    _ => State::Invalid,
                },
                State::ColEnd => {
                    range.end = line.start_offset;
                    col_defs.push((range.clone(), table::Alignment::from(flags)));
                    line.next();
                    range.start = line.start_offset;
                    range.end = line.start_offset;
                    flags.0 = false;
                    flags.1 = false;
                    State::ColStart
                }
                State::Invalid => break,
            }
        }
        if matches!(state, State::Col) {
            range.end = line.start_offset;
            col_defs.push((range, table::Alignment::from(flags)))
        }

        let optional = {
            if !col_defs.is_empty() {
                !col_defs
                    .first()
                    .zip(col_defs.last())
                    .map(|(a, b)| (a.0.start.saturating_sub(1), b.0.end))
                    .map(|(a, b)| {
                        line.get_raw(a)
                            .map(|it| it == &Token::Pipe)
                            .unwrap_or(false)
                            && line
                                .get_raw(b)
                                .map(|it| it == &Token::Pipe)
                                .unwrap_or(false)
                    })
                    .unwrap_or(false)
            } else {
                true
            }
        };
        if col_defs.len() > 1 || (col_defs.len() == 1 && !optional) {
            Some(col_defs)
        } else {
            None
        }
    }
    fn parse_columns<'input>(line: &Line<'input>) -> Option<Row<'input>> {
        let mut row: Row = (
            (line.start_location(), line.last_token_end_location()),
            Vec::new(),
        );
        let mut ranges = Vec::new();
        let mut range = Range { start: 0, end: 0 };
        let len = line.len();
        for (idx, item) in line.iter().enumerate() {
            if idx == 0 && item.token == Token::Pipe {
                range.start = 1;
                continue;
            }
            if idx + 1 == len {
                if item.token == Token::Pipe {
                    range.end = idx;
                } else {
                    range.end = len;
                }
                ranges.push(range);
                break;
            }
            if item.token == Token::Pipe {
                range.end = idx;
                ranges.push(range.clone());
                range.start = idx + 1;
                range.end = range.start;
                continue;
            }
        }
        let optional = {
            if !ranges.is_empty() {
                !ranges
                    .first()
                    .zip(ranges.last())
                    .map(|(a, b)| (a.start.saturating_sub(1), b.end))
                    .map(|(a, b)| {
                        line.get_raw(a)
                            .map(|it| it == &Token::Pipe)
                            .unwrap_or(false)
                            && line
                                .get_raw(b)
                                .map(|it| it == &Token::Pipe)
                                .unwrap_or(false)
                    })
                    .unwrap_or(false)
            } else {
                true
            }
        };
        row.1.extend(
            ranges
                .iter()
                .map(|range| line.slice(range.start, range.end).trim()),
        );
        let len = row.1.len();
        if len > 1 || (len == 1 && !optional) {
            Some(row)
        } else {
            None
        }
    }
}
impl BlockStrategy for table::Table {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        // 匹配表格第二行，忽略缩进
        line.skip(line.indent);
        if !line.validate(0, |it: &Token| {
            matches!(it, Token::Pipe | Token::Hyphen | Token::Colon)
        }) {
            return BlockMatching::Unmatched;
        }
        if parser.current_proc().body != MarkdownNode::Paragraph
            || !parser
                .inlines
                .get(&parser.curr_proc_node)
                .map(|it| it.len() == 1)
                .unwrap_or(false)
        {
            return BlockMatching::Unmatched;
        }
        let snapshot = line.snapshot();
        let col_defs = match Self::scan_columns(line) {
            Some(columns) => columns,
            None => return BlockMatching::Unmatched,
        };
        line.resume(snapshot);
        let table = MarkdownNode::Table(table::Table {
            column: col_defs.len(),
        });
        let header_cols =
            match Self::parse_columns(&parser.inlines.get(&parser.curr_proc_node).unwrap()[0]) {
                Some(it) if it.1.len() == col_defs.len() => {
                    parser.inlines.remove(&parser.curr_proc_node);
                    it
                }
                _ => return BlockMatching::Unmatched,
            };
        parser.replace_block(table, line.last_token_end_location());
        // 写入表头
        let idx = parser.append_block(MarkdownNode::TableHead, header_cols.0 .0);
        // 写入表头列
        for (idx, column) in header_cols.1.into_iter().enumerate() {
            let idx = parser.append_block(
                MarkdownNode::TableHeadCol(col_defs[idx].1.clone()),
                column.start_location(),
            );
            parser.finalize(idx, column.last_token_end_location());
            parser.inlines.insert(idx, vec![column]);
        }
        parser.finalize(idx, header_cols.0 .1);
        // 全部消耗
        line.skip_to_end();
        // 标记为全部未处理节点已关闭，防止 Table 节点被当作未匹配节点关闭
        parser.all_closed = true;
        BlockMatching::MatchedLeaf
    }

    fn process(ProcessCtx { line, parser, id }: ProcessCtx) -> BlockProcessing {
        let id = if let MarkdownNode::Table(_) = &parser.tree[id].body {
            parser.append_block(MarkdownNode::TableBody, line.start_location())
        } else {
            id
        };
        if MarkdownNode::TableBody != parser.tree[id].body {
            return BlockProcessing::Unprocessed;
        };
        let column =
            if let MarkdownNode::Table(table) = &parser.tree[parser.tree.get_parent(id)].body {
                table.column
            } else {
                return BlockProcessing::Unprocessed;
            };
        let row = match Self::parse_columns(line) {
            Some(row) => row,
            None => return BlockProcessing::Unprocessed,
        };
        let row_id = parser.append_block(MarkdownNode::TableRow, row.0 .0);
        let mut inserted = 0;
        for col in row.1.into_iter().take(column) {
            let idx = parser.append_block(MarkdownNode::TableDataCol, col.start_location());
            parser.finalize(idx, col.last_token_end_location());
            parser.inlines.insert(idx, vec![col]);
            inserted += 1;
        }
        // 为表格填充空白列
        let end_location = row.0 .1;
        for _ in 0..column.saturating_sub(inserted) {
            let idx = parser.append_block(MarkdownNode::TableDataCol, end_location);
            parser.finalize(idx, end_location);
        }
        // 关闭当前行
        parser.finalize(row_id, row.0 .1);
        BlockProcessing::Processed
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::table::{Alignment, Table};
    use crate::line::Line;
    use crate::parser::Parser;
    use crate::tokenizer::{Location, Tokenizer};

    #[test]
    fn case_temp() {
        let text = r#"65
        
| Left-aligned text | Center-aligned text | Right-aligned text |
| :---------------- | :-----------------: | -----------------: |
| Content           |       Content       |           RContent |"#;
        let ast = Parser::new(text).parse();
        println!("{ast:?}");
        // let text = r#"-- | --"#;
        // let ast = Parser::new(text).
    }

    #[test]
    fn test_scan_columns() {
        let text = r#"| --- | --- |"#;
        let mut line = Line::extract(&mut Tokenizer::new(text).tokenize()).unwrap();
        let col_defs = Table::scan_columns(&mut line).unwrap();
        assert_eq!(col_defs.len(), 2);
        assert_eq!(col_defs[0].1, Alignment::Left);
        assert_eq!(col_defs[1].1, Alignment::Left);

        let text = r#"-- | --"#;
        let mut line = Line::extract(&mut Tokenizer::new(text).tokenize()).unwrap();
        let col_defs = Table::scan_columns(&mut line).unwrap();
        assert_eq!(col_defs.len(), 2);
        assert_eq!(col_defs[0].1, Alignment::Left);
        assert_eq!(col_defs[1].1, Alignment::Left);

        let text = r#":-: | -----------:"#;
        let mut line = Line::extract(&mut Tokenizer::new(text).tokenize()).unwrap();
        let col_defs = Table::scan_columns(&mut line).unwrap();
        assert_eq!(col_defs.len(), 2);
        assert_eq!(col_defs[0].1, Alignment::Center);
        assert_eq!(col_defs[1].1, Alignment::Right);

        let text = r#":-- | :--: | --:"#;
        let mut line = Line::extract(&mut Tokenizer::new(text).tokenize()).unwrap();
        let col_defs = Table::scan_columns(&mut line).unwrap();
        assert_eq!(col_defs.len(), 3);
        assert_eq!(col_defs[0].1, Alignment::Left);
        assert_eq!(col_defs[1].1, Alignment::Center);
        assert_eq!(col_defs[2].1, Alignment::Right);

        let text = r#"| - |"#;
        let mut line = Line::extract(&mut Tokenizer::new(text).tokenize()).unwrap();
        let col_defs = Table::scan_columns(&mut line);
        assert!(col_defs.is_some());
        assert_eq!(col_defs.unwrap().len(), 1);

        let text = r#"-"#;
        let mut line = Line::extract(&mut Tokenizer::new(text).tokenize()).unwrap();
        assert!(Table::scan_columns(&mut line).is_none());

        let text = r#"|-"#;
        let mut line = Line::extract(&mut Tokenizer::new(text).tokenize()).unwrap();
        assert!(Table::scan_columns(&mut line).is_none());

        let text = r#"-|"#;
        let mut line = Line::extract(&mut Tokenizer::new(text).tokenize()).unwrap();
        assert!(Table::scan_columns(&mut line).is_none());
    }
    #[test]
    fn test_parse_columns() {
        let text = r#"| Left-aligned text | Center-aligned text | Right-aligned text |"#;
        let mut line = Line::extract(&mut Tokenizer::new(text).tokenize()).unwrap();
        let row = Table::parse_columns(&mut line);
        assert!(row.is_some());
        let row = row.unwrap();
        assert_eq!(row.0 .1, Location::new(1, 65));
        let cols = row.1.iter().map(|it| it.to_string()).collect::<Vec<_>>();
        assert_eq!(cols[0], " Left-aligned text ");
        assert_eq!(cols[1], " Center-aligned text ");
        assert_eq!(cols[2], " Right-aligned text ");

        let text = r#"| Left-aligned text |"#;
        let mut line = Line::extract(&mut Tokenizer::new(text).tokenize()).unwrap();
        let row = Table::parse_columns(&mut line);
        assert!(row.is_some());

        let text = r#"Left-aligned text |"#;
        let mut line = Line::extract(&mut Tokenizer::new(text).tokenize()).unwrap();
        let row = Table::parse_columns(&mut line);
        assert!(row.is_none());
    }
}
