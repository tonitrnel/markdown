use crate::ast::{table, MarkdownNode};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};
use crate::parser::Location;
use crate::span::Span;
use std::ops::Range;

type Row<'input> = ((Location, Location), Vec<Span<'input>>);
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
    fn scan_columns(line: &mut Span) -> Option<ColDefs> {
        let mut col_defs: ColDefs = Vec::new();
        let mut range = Range { start: 0, end: 0 };
        let mut flags = (false, false);
        let mut has_pipe_start = false;
        let mut has_pipe_end = false;
        let mut in_col = false;
        let mut col_started = false;

        // State machine for scanning delimiter row
        let snapshot = line.snapshot();
        let mut first = true;
        while let Some(b) = line.peek() {
            match b {
                b'|' if first => {
                    has_pipe_start = true;
                    line.next_byte();
                    range.start = 0; // will be set properly
                    first = false;
                    col_started = true;
                    continue;
                }
                b'|' if in_col || col_started => {
                    range.end = 0; // placeholder
                    col_defs.push((range.clone(), table::Alignment::from(flags)));
                    line.next_byte();
                    has_pipe_end = true;
                    flags = (false, false);
                    in_col = false;
                    col_started = true;
                    continue;
                }
                b' ' | b'\t' => {
                    line.next_byte();
                    first = false;
                    continue;
                }
                b':' if !in_col => {
                    flags.0 = true;
                    line.next_byte();
                    first = false;
                    col_started = true;
                    continue;
                }
                b'-' => {
                    in_col = true;
                    line.next_byte();
                    first = false;
                    col_started = true;
                    continue;
                }
                b':' if in_col => {
                    flags.1 = true;
                    line.next_byte();
                    continue;
                }
                _ => {
                    // Invalid character in delimiter row
                    line.resume(&snapshot);
                    return None;
                }
            }
        }
        // Handle last column
        if in_col {
            col_defs.push((range, table::Alignment::from(flags)));
        }

        let optional = has_pipe_start && has_pipe_end;
        if col_defs.len() > 1 || (col_defs.len() == 1 && optional) {
            line.resume(&snapshot);
            Some(col_defs)
        } else {
            line.resume(&snapshot);
            None
        }
    }

    fn parse_columns<'input>(line: &Span<'input>) -> Option<Row<'input>> {
        let mut row: Row = (
            (line.start_location(), line.last_token_end_location()),
            Vec::new(),
        );
        let mut ranges: Vec<Range<usize>> = Vec::new();
        let mut range_start = 0;
        let len = line.len();
        let mut has_pipe_start = false;
        let mut has_pipe_end = false;
        let mut has_any_pipe = false;

        let mut idx = 0;
        while idx < len {
            let b = match line.get(idx) {
                Some(b) => b,
                None => break,
            };
            match b {
                b'\\' if idx + 1 < len => {
                    // Skip escaped character (e.g. \|)
                    idx += 2;
                    continue;
                }
                b'|' if idx == 0 => {
                    has_pipe_start = true;
                    has_any_pipe = true;
                    range_start = 1;
                    idx += 1;
                    continue;
                }
                b'|' => {
                    has_any_pipe = true;
                    ranges.push(Range {
                        start: range_start,
                        end: idx,
                    });
                    range_start = idx + 1;
                    has_pipe_end = true;
                    idx += 1;
                    continue;
                }
                _ => {
                    idx += 1;
                    if idx == len {
                        ranges.push(Range {
                            start: range_start,
                            end: len,
                        });
                    }
                    continue;
                }
            }
        }

        let optional = has_pipe_start && has_pipe_end;
        row.1.extend(
            ranges
                .iter()
                .map(|range| line.slice(range.start, range.end).trim()),
        );
        let col_count = row.1.len();
        if col_count > 1 || (col_count == 1 && (optional || !has_any_pipe)) {
            Some(row)
        } else {
            None
        }
    }

    pub(crate) fn reprocess(ProcessCtx { line, parser, id }: ProcessCtx) -> bool {
        let table_id = {
            if parser.tree[id].body != MarkdownNode::TableBody {
                parser.append_block(MarkdownNode::TableBody, line.start_location());
                id
            } else {
                parser.tree.get_parent(id)
            }
        };
        let column = if let MarkdownNode::Table(table) = &parser.tree[table_id].body {
            table.column
        } else {
            return false;
        };
        let start_location = line.start_location();
        let end_location = line.last_token_end_location();
        let row_id = parser.append_block(MarkdownNode::TableRow, start_location);
        let col_id = parser.append_block(MarkdownNode::TableDataCol, start_location);
        let col = line.slice(0, line.len());
        parser.finalize(col_id, end_location);
        parser.inlines.insert(col_id, vec![col]);
        for _ in 1..column {
            let col_id = parser.append_block(MarkdownNode::TableDataCol, end_location);
            parser.finalize(col_id, end_location);
        }
        parser.finalize(row_id, end_location);
        line.skip_to_end();
        true
    }
}

impl BlockStrategy for table::Table {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        // Match table second row (delimiter), ignore indent
        line.skip(line.indent_len());
        // Must start with pipe, hyphen, or colon
        match line.peek() {
            Some(b'|') | Some(b'-') | Some(b':') => {}
            _ => return BlockMatching::Unmatched,
        }
        if parser.current_proc().body != MarkdownNode::Paragraph {
            return BlockMatching::Unmatched;
        }
        let paragraph_id = parser.curr_proc_node;
        let paragraph_line_count = parser
            .inlines
            .get(&paragraph_id)
            .map(|it| it.len())
            .unwrap_or(0);
        if paragraph_line_count == 0 {
            return BlockMatching::Unmatched;
        }
        let snapshot = line.snapshot();
        let col_defs = match Self::scan_columns(line) {
            Some(columns) => columns,
            None => return BlockMatching::Unmatched,
        };
        line.resume(&snapshot);
        let col_len = col_defs.len();
        let Some(header_source) = parser
            .inlines
            .get(&paragraph_id)
            .and_then(|it| it.last().cloned())
        else {
            return BlockMatching::Unmatched;
        };
        let table = MarkdownNode::Table(Box::new(table::Table {
            column: col_len,
            alignments: col_defs.into_iter().map(|it| it.1).collect(),
        }));
        let header_cols = match Self::parse_columns(&header_source) {
            Some(it) if it.1.len() == col_len => it,
            _ => return BlockMatching::Unmatched,
        };
        if paragraph_line_count == 1 {
            parser.inlines.remove(&paragraph_id);
            parser.replace_block(table, line.last_token_end_location());
        } else {
            if let Some(lines) = parser.inlines.get_mut(&paragraph_id) {
                lines.pop();
                if let Some(last) = lines.last() {
                    parser.tree[paragraph_id].end = last.last_token_end_location();
                }
            }
            let paragraph_end = parser.tree[paragraph_id].end;
            parser.finalize(paragraph_id, paragraph_end);
            parser.append_block(table, header_source.start_location());
        }
        // Write table header
        let idx = parser.append_block(MarkdownNode::TableHead, header_cols.0 .0);
        let row_idx = parser.append_block(MarkdownNode::TableRow, header_cols.0 .0);
        for column in header_cols.1.into_iter() {
            let idx = parser.append_block(MarkdownNode::TableHeadCol, column.start_location());
            parser.finalize(idx, column.last_token_end_location());
            parser.inlines.insert(idx, vec![column]);
        }
        parser.finalize(row_idx, header_cols.0 .1);
        parser.finalize(idx, header_cols.0 .1);
        line.skip_to_end();
        parser.all_closed = true;
        BlockMatching::MatchedLeaf
    }

    fn process(ProcessCtx { line, parser, id }: ProcessCtx) -> BlockProcessing {
        if let Some(maybe_body_idx) = parser.tree.get_last_child(id) {
            if !matches!(parser.tree[maybe_body_idx].body, MarkdownNode::TableBody) {
                parser.append_block(MarkdownNode::TableBody, line.start_location());
            }
        } else {
            parser.append_block(MarkdownNode::TableBody, line.start_location());
        }
        let column = if let MarkdownNode::Table(table) = &parser.tree[id].body {
            table.column
        } else {
            return BlockProcessing::Unprocessed;
        };
        if line.is_blank() {
            return BlockProcessing::Unprocessed;
        }
        // Per GFM spec: table is broken at beginning of another block-level structure.
        // If line starts with '>' (blockquote marker) and has no pipe, break the table.
        if !line.is_indented() {
            let snap = line.snapshot();
            line.advance_next_nonspace();
            let first = line.peek();
            line.resume(&snap);
            if first == Some(b'>') {
                return BlockProcessing::Unprocessed;
            }
        }
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
        let end_location = row.0 .1;
        for _ in inserted..column {
            let idx = parser.append_block(MarkdownNode::TableDataCol, end_location);
            parser.finalize(idx, end_location);
        }
        parser.finalize(row_id, row.0 .1);
        BlockProcessing::Processed
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn gfm_case_198() {
        let ast = Parser::new(
            r#"| foo | bar |
| --- | --- |
| baz | bim |"#,
        )
        .parse();
        assert_eq!(
            ast.to_html(),
            r#"<table>
<thead>
<tr>
<th>foo</th>
<th>bar</th>
</tr>
</thead>
<tbody>
<tr>
<td>baz</td>
<td>bim</td>
</tr>
</tbody>
</table>"#
        )
    }

    #[test]
    fn gfm_case_199() {
        let ast = Parser::new(
            r#"| abc | defghi |
:-: | -----------:
bar | baz"#,
        )
        .parse();
        println!("AST:\n{ast:?}");
        assert_eq!(
            ast.to_html(),
            r#"<table>
<thead>
<tr>
<th style="text-align: center">abc</th>
<th style="text-align: right">defghi</th>
</tr>
</thead>
<tbody>
<tr>
<td style="text-align: center">bar</td>
<td style="text-align: right">baz</td>
</tr>
</tbody>
</table>"#
        );
    }

    #[test]
    fn gfm_case_200() {
        let ast = Parser::new(
            r#"| f\|oo  |
| ------ |
| b `\|` az |
| b **\|** im |"#,
        )
        .parse();
        assert_eq!(
            ast.to_html(),
            r#"<table>
<thead>
<tr>
<th>f|oo</th>
</tr>
</thead>
<tbody>
<tr>
<td>b <code>|</code> az</td>
</tr>
<tr>
<td>b <strong>|</strong> im</td>
</tr>
</tbody>
</table>"#
        );
    }

    #[test]
    fn gfm_case_201() {
        let ast = Parser::new(
            r#"| abc | def |
| --- | --- |
| bar | baz |
> bar"#,
        )
        .parse();
        assert_eq!(
            ast.to_html(),
            r#"<table>
<thead>
<tr>
<th>abc</th>
<th>def</th>
</tr>
</thead>
<tbody>
<tr>
<td>bar</td>
<td>baz</td>
</tr>
</tbody>
</table>
<blockquote>
<p>bar</p>
</blockquote>"#
        );
    }

    #[test]
    fn gfm_case_202() {
        let ast = Parser::new(
            r#"| abc | def |
| --- | --- |
| bar | baz |
bar

bar"#,
        )
        .parse();
        assert_eq!(
            ast.to_html(),
            r#"<table>
<thead>
<tr>
<th>abc</th>
<th>def</th>
</tr>
</thead>
<tbody>
<tr>
<td>bar</td>
<td>baz</td>
</tr>
<tr>
<td>bar</td>
<td></td>
</tr>
</tbody>
</table>
<p>bar</p>"#
        );
    }

    #[test]
    fn gfm_case_203() {
        let ast = Parser::new(
            r#"| abc | def |
| --- |
| bar |"#,
        )
        .parse();
        assert_eq!(
            ast.to_html(),
            r#"<p>| abc | def |
| --- |
| bar |</p>"#
        );
    }

    #[test]
    fn gfm_case_204() {
        let ast = Parser::new(
            r#"| abc | def |
| --- | --- |
| bar |
| bar | baz | boo |"#,
        )
        .parse();
        assert_eq!(
            ast.to_html(),
            r#"<table>
<thead>
<tr>
<th>abc</th>
<th>def</th>
</tr>
</thead>
<tbody>
<tr>
<td>bar</td>
<td></td>
</tr>
<tr>
<td>bar</td>
<td>baz</td>
</tr>
</tbody>
</table>"#
        );
    }

    #[test]
    fn gfm_case_205() {
        let ast = Parser::new(
            r#"| abc | def |
| --- | --- |"#,
        )
        .parse();
        assert_eq!(
            ast.to_html(),
            r#"<table>
<thead>
<tr>
<th>abc</th>
<th>def</th>
</tr>
</thead>
</table>"#
        );
    }
}
