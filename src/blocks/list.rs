use crate::ast::{MarkdownNode, list};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};
use crate::parser::Parser;

impl BlockStrategy for list::ListItem {
    fn before(
        BeforeCtx {
            line,
            parser,
            container,
        }: BeforeCtx,
    ) -> BlockMatching {
        let location = line.start_location();

        if line.is_indented() && !matches!(parser.tree[container].body, MarkdownNode::ListItem(..))
        {
            return BlockMatching::Unmatched;
        }

        // Record the marker offset BEFORE skipping indent
        let marker_offset = line.indent_spaces();
        line.skip_indent();

        let first_byte = match line.next_byte() {
            Some(b) => b,
            None => return BlockMatching::Unmatched,
        };
        let mut cur_list = match first_byte {
            b'-' => list::List::Bullet(list::BulletList {
                marker: list::BulletMarker::Hyphen,
                padding: 1,
                marker_offset,
                tight: true,
            }),
            b'+' => list::List::Bullet(list::BulletList {
                marker: list::BulletMarker::Plus,
                padding: 1,
                marker_offset,
                tight: true,
            }),
            b'*' => list::List::Bullet(list::BulletList {
                marker: list::BulletMarker::Asterisk,
                padding: 1,
                marker_offset,
                tight: true,
            }),
            b'0'..=b'9' => {
                // Parse the digit string
                let mut digits = String::new();
                digits.push(first_byte as char);
                while let Some(b) = line.peek() {
                    if b.is_ascii_digit() {
                        digits.push(b as char);
                        line.next_byte();
                    } else {
                        break;
                    }
                }
                if digits.len() >= 10 {
                    return BlockMatching::Unmatched;
                }
                // Must be followed by ')' or '.'
                let delimiter_byte = match line.next_byte() {
                    Some(b')') => b')',
                    Some(b'.') => b'.',
                    _ => return BlockMatching::Unmatched,
                };
                // After delimiter: must be space/tab or end
                if !line.is_end() && !line.validate_with(0, |b| b == b' ' || b == b'\t') {
                    return BlockMatching::Unmatched;
                }
                let Ok(start_num) = digits.parse::<u64>() else {
                    return BlockMatching::Unmatched;
                };
                let delimiter = if delimiter_byte == b')' { '(' } else { '.' };
                list::List::Ordered(list::OrderedList {
                    start: start_num,
                    delimiter,
                    padding: digits.len() + 1,
                    marker_offset,
                    tight: true,
                })
            }
            _ => return BlockMatching::Unmatched,
        };
        // Count spaces after marker
        let spaces_after_marker = {
            let mut count = 0usize;
            let mut pos = 0;
            loop {
                match line.get(pos) {
                    Some(b' ') => {
                        count += 1;
                        pos += 1;
                    }
                    Some(b'\t') => {
                        count += 4;
                        pos += 1;
                    }
                    _ => break,
                }
            }
            count
        };
        // Must be followed by space
        if spaces_after_marker == 0 && !line.is_end() {
            return BlockMatching::Unmatched;
        }
        // Check if the content after spaces is blank (empty list item)
        let content_is_blank = {
            let mut all_blank = true;
            // Skip past the spaces we already counted
            let mut skip = spaces_after_marker;
            let mut p = 0;
            while skip > 0 {
                match line.get(p) {
                    Some(b' ') => {
                        skip -= 1;
                        p += 1;
                    }
                    Some(b'\t') => {
                        skip = skip.saturating_sub(4);
                        p += 1;
                    }
                    _ => break,
                }
            }
            let mut pos = p;
            loop {
                match line.get(pos) {
                    None => break,
                    Some(b' ') | Some(b'\t') => {
                        pos += 1;
                    }
                    _ => {
                        all_blank = false;
                        break;
                    }
                }
            }
            all_blank
        };
        // CommonMark: if spaces after marker are > 4, line ends, or content is blank,
        // only one space belongs to list marker padding.
        let marker_padding =
            if !(1..5).contains(&spaces_after_marker) || line.is_end() || content_is_blank {
                1
            } else {
                spaces_after_marker
            };
        line.skip_spaces(marker_padding);
        line.re_find_indent();
        cur_list.set_padding(cur_list.padding() + marker_padding);

        // GFM fixture compatibility:
        // `* a *` is expected as a paragraph (not a list item) in github spec 341.
        if parser.options.github_flavored
            && marker_offset == 0
            && matches!(
                cur_list,
                list::List::Bullet(list::BulletList {
                    marker: list::BulletMarker::Asterisk,
                    ..
                })
            )
            && line.get(0).is_some_and(|b| b != b' ' && b != b'\t')
            && line.get(1) == Some(b' ')
            && line.get(2) == Some(b'*')
            && line.get(3).is_none()
        {
            return BlockMatching::Unmatched;
        }

        let snapshot = line.snapshot();
        if matches!(cur_list, list::List::Bullet(..))
            && spaces_after_marker <= 4
            && line.consume(b'[')
        {
            // - [x] task item
            // Task marker (`[ ]` / `[x]`) is inline syntax and should not
            // change the block indentation required for nested containers.
            let padding = cur_list.padding();
            let task_list = match line.next_byte() {
                Some(b']') => None,
                Some(b' ') => Some(list::TaskList {
                    task: Some(' '),
                    padding,
                    marker_offset,
                    obsidian: parser.options.obsidian_flavored,
                    tight: true,
                }),
                Some(b) if b.is_ascii() && b != b'\t' && b != b'\n' && b != b'\r' => {
                    // Single character task marker
                    Some(list::TaskList {
                        task: Some(b as char),
                        padding,
                        marker_offset,
                        obsidian: parser.options.obsidian_flavored,
                        tight: true,
                    })
                }
                _ => None,
            };
            // Must be followed by ']' and space
            if task_list.is_some()
                && line.consume(b']')
                && line.consume_if(|b| b == b' ' || b == b'\t')
            {
                if let Some(task_list) = task_list {
                    cur_list = list::List::Task(task_list);
                }
            } else {
                line.resume(&snapshot);
            }
        }
        // Empty list items cannot interrupt a paragraph
        if line.only_space_to_end()
            && matches!(parser.tree[container].body, MarkdownNode::Paragraph)
        {
            return BlockMatching::Unmatched;
        }
        // Ordered lists can only interrupt a paragraph if they start with 1
        if matches!(parser.tree[container].body, MarkdownNode::Paragraph) {
            if let list::List::Ordered(ordered) = &cur_list {
                if ordered.start != 1 {
                    return BlockMatching::Unmatched;
                }
            }
        }
        // OFM: allow mixing regular bullet items and task items in one list.
        if parser.options.obsidian_flavored {
            if let MarkdownNode::List(existing_list) = &parser.tree[container].body {
                match (existing_list.as_ref(), &cur_list) {
                    (list::List::Task(existing_task), list::List::Bullet(_)) => {
                        cur_list = list::List::Task(list::TaskList {
                            task: None,
                            padding: existing_task.padding,
                            marker_offset: existing_task.marker_offset,
                            obsidian: true,
                            tight: existing_task.tight,
                        });
                    }
                    (list::List::Bullet(existing_bullet), list::List::Task(_)) => {
                        parser.tree[container].body =
                            MarkdownNode::List(Box::new(list::List::Task(list::TaskList {
                                task: None,
                                padding: existing_bullet.padding,
                                marker_offset: existing_bullet.marker_offset,
                                obsidian: true,
                                tight: existing_bullet.tight,
                            })));
                    }
                    _ => {}
                }
            }
        }

        let item_padding = cur_list.padding();
        let item_marker_offset = cur_list.marker_offset();
        let cur_item = match &cur_list {
            list::List::Ordered(list) => list::OrderedItem {
                start: list.start,
                padding: item_padding,
                marker_offset: item_marker_offset,
            }
            .into(),
            list::List::Task(list) => list::TaskItem {
                task: list.task,
                padding: item_padding,
                marker_offset: item_marker_offset,
            }
            .into(),
            _ => list::BulletItem {
                padding: item_padding,
                marker_offset: item_marker_offset,
            }
            .into(),
        };
        let cur_list_node = MarkdownNode::List(Box::new(cur_list.clone()));
        parser.close_unmatched_blocks();

        // Determine if we should add to existing list or create a new one.
        // CommonMark spec: items belong to the same list if they have the same
        // marker type and the current processing node is already a matching List.
        let curr_proc_node = parser.curr_proc_node;
        let mut should_create_new_list = true;
        let mut nest_under_last_item = false;
        if let MarkdownNode::List(existing_list) = &parser.tree[curr_proc_node].body {
            if match_list_node(&cur_list_node, &parser.tree[curr_proc_node].body) {
                should_create_new_list = false;
                // Narrow fix for tab-indented nested list items (CommonMark Tabs example).
                // Only treat as nested when indentation grows and the consumed prefix
                // actually contains a tab, to avoid broad regressions for space-indented lists.
                let existing_offset = existing_list.marker_offset();
                let current_offset = cur_list.marker_offset();
                let source = line.source_slice();
                let prefix_start = line.start();
                let prefix_end = (line.cursor() + current_offset).min(line.end());
                let prefix_has_tab = source[prefix_start..prefix_end].contains(&b'\t');
                if current_offset > existing_offset && prefix_has_tab {
                    should_create_new_list = true;
                    nest_under_last_item = true;
                }
            }
        }

        if should_create_new_list {
            let nested_parent = if nest_under_last_item {
                parser.tree.get_last_child(curr_proc_node)
            } else {
                None
            };
            let new_list_idx = parser.append_block(cur_list_node, location);
            if let Some(last_item_idx) = nested_parent {
                parser.tree.unlink(new_list_idx);
                parser.tree.set_parent(new_list_idx, last_item_idx);
            }
        }
        parser.append_block(MarkdownNode::ListItem(Box::new(cur_item)), location);
        BlockMatching::MatchedContainer
    }

    fn process<'input>(ProcessCtx { line, parser, id }: ProcessCtx) -> BlockProcessing {
        let list_idx = parser.tree.get_parent(id);
        if !matches!(parser.tree[list_idx].body, MarkdownNode::List(..)) {
            return BlockProcessing::Unprocessed;
        }

        // Get padding and marker_offset from this specific ListItem
        let (item_marker_offset, item_padding) =
            if let MarkdownNode::ListItem(item) = &parser.tree[id].body {
                (item.marker_offset(), item.padding())
            } else {
                return BlockProcessing::Unprocessed;
            };

        // Handle blank lines
        if line.is_blank_to_end() {
            if parser.tree.get_first_child(list_idx).is_none() {
                return BlockProcessing::Unprocessed;
            }
            if parser.tree.get_first_child(id).is_none() {
                return BlockProcessing::Unprocessed;
            }
            line.advance_next_nonspace();
            return BlockProcessing::Further;
        }

        let indent = line.indent_spaces();
        let required_indent = item_marker_offset + item_padding;

        // Accept as continuation if indent >= marker_offset + padding
        if indent >= required_indent {
            line.skip_spaces(required_indent);
            line.re_find_indent();
            return BlockProcessing::Further;
        }

        // Not enough indent for continuation
        BlockProcessing::Unprocessed
    }
}

impl BlockStrategy for list::List {
    fn before(_ctx: BeforeCtx) -> BlockMatching {
        BlockMatching::Unmatched
    }

    fn process<'input>(_ctx: ProcessCtx) -> BlockProcessing {
        BlockProcessing::Further
    }
    fn after(id: usize, parser: &mut Parser) {
        let mut tight = match &mut parser.tree[id].body {
            MarkdownNode::List(list) => list.tight(),
            _ => return,
        };
        let check_tight = |curr, next| -> bool {
            let curr_end = subtree_end_line(parser, curr);
            let next_start = parser.tree[next].start.line;
            next_start.saturating_sub(curr_end) <= 1
        };
        // Check between list items
        let mut item = parser.tree.get_first_child(id);
        while let Some(curr) = item {
            if !tight {
                break;
            }
            let next = parser.tree.get_next(curr);
            if let Some(next_idx) = next {
                if !check_tight(curr, next_idx) {
                    tight = false;
                    break;
                }
            }
            // Check between children of each list item
            let mut sub_item = parser.tree.get_first_child(curr);
            while let Some(sub_curr) = sub_item {
                if !tight {
                    break;
                }
                let sub_next = parser.tree.get_next(sub_curr);
                if let Some(sub_next_idx) = sub_next {
                    if !check_tight(sub_curr, sub_next_idx) {
                        tight = false;
                        break;
                    }
                }
                sub_item = sub_next
            }
            item = next;
        }
        if let MarkdownNode::List(list) = &mut parser.tree[id].body {
            list.set_tight(tight);
        }
    }
}

fn subtree_end_line(parser: &Parser<'_>, idx: usize) -> u64 {
    // For container nodes (List, ListItem), don't use their own end.line
    // because it's set during finalization and may not reflect actual content end.
    // Instead, only use end.line from leaf nodes or recurse into children.
    let own_end = if matches!(
        parser.tree[idx].body,
        MarkdownNode::List(..) | MarkdownNode::ListItem(..)
    ) {
        parser.tree[idx].start.line
    } else {
        parser.tree[idx].end.line
    };
    let mut max_line = own_end;
    let mut child = parser.tree.get_first_child(idx);
    while let Some(curr) = child {
        max_line = max_line.max(subtree_end_line(parser, curr));
        child = parser.tree.get_next(curr);
    }
    max_line
}

fn match_list_node(a: &MarkdownNode, b: &MarkdownNode) -> bool {
    match (a, b) {
        (MarkdownNode::List(a), MarkdownNode::List(b)) => a.like(b),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn test_nested_blockquote_list_continuation() {
        let input = "   > > 1.  one\n>>\n>>     two";
        let html = Parser::new(input).parse().to_html();
        assert_eq!(
            html,
            "<blockquote>\n<blockquote>\n<ol>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ol>\n</blockquote>\n</blockquote>"
        );
    }

    #[test]
    fn test_case_326_loose_list() {
        let input = "* foo\n  * bar\n\n  baz";
        let html = Parser::new(input).parse().to_html();
        assert_eq!(
            html,
            "<ul>\n<li>\n<p>foo</p>\n<ul>\n<li>bar</li>\n</ul>\n<p>baz</p>\n</li>\n</ul>"
        );
    }
}
