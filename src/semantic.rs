//! 语义准备（F2）：引用定义提取与 OFM BlockId 发现。
//!
//! 完整解析（`Parser::parse_inlines`）与选择性解析的语义准备阶段共用本模块，
//! 幂等标志保证两条路径先后调用只生效一次。

use crate::parser::Parser;

impl<'input> Parser<'input> {
    /// 对整个已接受前缀提取 link reference definitions（幂等）。
    ///
    /// 内部会消费纯定义段落（`tree.remove`）并裁剪前缀定义行，不可重复执行。
    pub(crate) fn prepare_reference_definitions(&mut self) {
        if self.reference_definitions_extracted {
            return;
        }
        self.reference_definitions_extracted = true;
        self.parse_reference_link();
    }

    /// 按文档顺序发现各 pending Block 的 OFM BlockId 并写入 `Node.id`（幂等）。
    ///
    /// 仅由语义准备阶段调用（选择前的目标可寻址性）；完整解析路径不需要它，
    /// inline 引擎在物化时就是 id 的权威写入者，避免为每个 pending 条目
    /// 支付冗余扫描。只做**发现**，不改写 pending spans：标记剥离仍完全由
    /// inline 引擎完成（`inlines::mod` 的 `^` 分派与 `inlines::newline` 的
    /// 行首形态），物化后引擎以相同值覆盖写 `Node.id`，输出逐字节一致。
    /// 发现范围是安全子集：仅检查每个条目**最后一个** span 的行尾形态
    /// （`^id` 独立行、行尾 ` ^id`、纯 `^id` Block），`^` 之后必须仅有
    /// `[A-Za-z0-9-]+` 与空白直至条目末尾——该校验天然拒绝多行 code span 等
    /// 引擎不可达位置的伪标记。块中（非末行）形态不在发现范围内，仅由引擎
    /// 在物化时识别；选择性模式下未物化 Block 的块中 id 因此不可寻址
    /// （Obsidian 文档语义即"块末"，记录于 map ticket 10）。
    pub(crate) fn discover_block_ids(&mut self) {
        if self.block_ids_discovered || !self.options.obsidian_flavored {
            self.block_ids_discovered = true;
            return;
        }
        self.block_ids_discovered = true;
        // 存储即文档顺序（P4），无需收集排序；`as_str` 基于 cursor，无需克隆规范化
        let node_ids: Vec<usize> = self.inlines.live_ids_in_document_order().collect();
        for node_id in node_ids {
            let Some(spans) = self.inlines.get(node_id) else {
                continue;
            };
            let Some(last) = spans.last() else {
                continue;
            };
            if let Some(id) = trailing_block_id(last.as_str()) {
                self.tree[node_id].id = Some(Box::new(id.to_string()));
                self.semantic_id_nodes.push(node_id);
            }
        }
    }
}

impl<'input> Parser<'input> {
    /// 选择性物化（F3）：按文档顺序物化给定的 pending 条目集合。
    pub(crate) fn materialize_pending_subset(&mut self, ids: &mut Vec<usize>) {
        ids.sort_unstable();
        for &id in ids.iter() {
            if self.reach_node_limit() {
                return;
            }
            let Some(spans) = self.inlines.remove(id) else {
                continue;
            };
            self.materialize_pending_entry(id, spans);
        }
    }

    /// 收集 `root` 子树内（含自身）仍处于 pending 状态的条目 id。
    pub(crate) fn collect_pending_in_subtree(&self, root: usize, out: &mut Vec<usize>) {
        let mut stack: Vec<usize> = Vec::new();
        let mut node = Some(root);
        while let Some(id) = node {
            if self.inlines.contains(id) {
                out.push(id);
            }
            if let Some(child) = self.tree.get_first_child(id) {
                stack.push(id);
                node = Some(child);
            } else {
                node = if id == root {
                    None
                } else {
                    self.tree.get_next(id)
                };
                while node.is_none() {
                    match stack.pop() {
                        Some(parent) if parent != root => node = self.tree.get_next(parent),
                        _ => break,
                    }
                }
            }
        }
    }

    /// 物化被引用 footnote definition 的 pending 内容（工作队列，处理嵌套引用）。
    pub(crate) fn materialize_footnote_dependencies(&mut self) {
        let mut processed: rustc_hash::FxHashSet<usize> = rustc_hash::FxHashSet::default();
        let mut pending_ids: Vec<usize> = Vec::new();
        loop {
            pending_ids.clear();
            // 已产生引用的 label 对应的 definition 子树中仍 pending 的条目
            for (label, _) in self.footnote_refs.iter() {
                if let Some(&def_id) = self.footnotes.get(label) {
                    if processed.contains(&def_id) {
                        continue;
                    }
                    self.collect_pending_in_subtree(def_id, &mut pending_ids);
                }
            }
            // 标记本轮涉及的 definition，避免重复扫描
            for (label, _) in self.footnote_refs.iter() {
                if let Some(&def_id) = self.footnotes.get(label) {
                    processed.insert(def_id);
                }
            }
            if pending_ids.is_empty() {
                return;
            }
            if self.reach_node_limit() {
                return;
            }
            self.materialize_pending_subset(&mut pending_ids);
        }
    }

    /// 语义目标收集（C3 增量版）：Heading 创建与 BlockId 发现时已按文档序记录，
    /// 此处仅合并、按 (span.start, 节点 id) 排序（等价文档前序：父先于子、
    /// 兄弟按位置）并去重（Heading 兼 BlockId 只产生一个目标）。
    pub(crate) fn collect_semantic_targets(&self) -> Vec<usize> {
        let mut targets: Vec<usize> =
            Vec::with_capacity(self.heading_nodes.len() + self.semantic_id_nodes.len());
        targets.extend(self.heading_nodes.iter().copied().filter(|&id| {
            !self.tree.is_free(id)
                && matches!(self.tree[id].body, crate::ast::MarkdownNode::Heading(_))
        }));
        targets.extend(
            self.semantic_id_nodes
                .iter()
                .copied()
                .filter(|&id| !self.tree.is_free(id) && self.tree[id].id.is_some()),
        );
        targets.sort_unstable_by_key(|&id| (self.tree[id].span.start, id));
        targets.dedup();
        targets
    }
}

/// 返回 `text` 行尾的 BlockId（若存在）。
///
/// 与 `inlines::link::process_block_id` 相同的字符集与 EOL 校验：
/// 最右侧 `^` 之后为 `[A-Za-z0-9-]+`，其后仅允许空白直至末尾。
fn trailing_block_id(text: &str) -> Option<&str> {
    let bytes = text.as_bytes();
    let caret = bytes.iter().rposition(|&b| b == b'^')?;
    let rest = &bytes[caret + 1..];
    let mut len = 0usize;
    while let Some(&b) = rest.get(len) {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' => len += 1,
            _ => break,
        }
    }
    if len == 0 {
        return None;
    }
    for &b in &rest[len..] {
        match b {
            b' ' | b'\t' | b'\n' | b'\r' => (),
            _ => return None,
        }
    }
    Some(unsafe { std::str::from_utf8_unchecked(&rest[..len]) })
}

#[cfg(test)]
mod tests {
    use super::trailing_block_id;

    #[test]
    fn accepts_trailing_forms() {
        assert_eq!(trailing_block_id("^abc"), Some("abc"));
        assert_eq!(trailing_block_id("text ^my-id"), Some("my-id"));
        assert_eq!(trailing_block_id("text ^my-id  \t"), Some("my-id"));
        assert_eq!(trailing_block_id("a ^first b ^second"), Some("second"));
    }

    #[test]
    fn rejects_non_terminal_forms() {
        assert_eq!(trailing_block_id("text ^id`"), None);
        assert_eq!(trailing_block_id("text ^id]]"), None);
        assert_eq!(trailing_block_id("text ^"), None);
        assert_eq!(trailing_block_id("no caret"), None);
        assert_eq!(trailing_block_id("^id trailing words"), None);
    }
}
