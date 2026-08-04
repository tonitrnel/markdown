//! Arena-backed tree storage used by parsed documents.

use std::fmt::Debug;
use std::num::NonZeroUsize;
use std::ops::{Index, IndexMut};

/// 兄弟/子链接的紧凑表示（P5）：槽位 0 恒为根节点、绝不作为任何节点的
/// child/sibling，因此链接可用 `Option<NonZeroUsize>`（8 字节，含 niche）
/// 取代 `Option<usize>`（16 字节），每槽节省 32 字节。
type Link = Option<NonZeroUsize>;

#[inline]
fn link_to(idx: usize) -> Link {
    debug_assert!(idx != 0, "根节点（槽 0）不可成为 child/sibling 链接目标");
    NonZeroUsize::new(idx)
}

#[inline]
fn link_get(link: Link) -> Option<usize> {
    link.map(NonZeroUsize::get)
}

#[derive(Debug, Clone, Copy)]
/// Internal tree slot containing an optional item and structural links.
///
/// Most users interact with [`Tree`] through node IDs rather than using this
/// type directly.
pub struct TreeNode<T> {
    /// Stored node value, or `None` after removal.
    pub item: Option<T>,
    parent: usize,
    first_child: Link,
    last_child: Link,
    next: Link,
    prev: Link,
}

impl<T: PartialEq> PartialEq<T> for TreeNode<T> {
    fn eq(&self, other: &T) -> bool {
        match self.item.as_ref() {
            Some(item) => item.eq(other),
            None => false,
        }
    }
}

impl<T> Default for TreeNode<T> {
    fn default() -> Self {
        Self {
            item: None,
            parent: 0,
            first_child: None,
            last_child: None,
            next: None,
            prev: None,
        }
    }
}

#[derive(Clone)]
/// An arena-backed tree addressed by stable integer node IDs.
///
/// In parsed Markdown documents, node ID `0` is the document root. Index the
/// tree with a node ID to access its value, and use the `get_*` methods to
/// navigate parent, child, and sibling relationships.
pub struct Tree<T> {
    /// 存储所有节点
    nodes: Vec<TreeNode<T>>,
    /// 存储已打开的分支分叉点的索引
    forks: Vec<usize>,
    /// 存储当前索引，它可能在树主干上，也可能在树分支上或者没有
    cur: Option<usize>,
    /// free 节点标志位（按节点 id 索引；P5 取代 FxHashSet）
    free_flags: Vec<bool>,
}
impl<T> Index<usize> for Tree<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(node) = self.nodes.index(index).item.as_ref() {
            node
        } else {
            panic!("Node #{index} has been released or has an invalid node index")
        }
    }
}
impl<T> IndexMut<usize> for Tree<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if let Some(node) = self.nodes.index_mut(index).item.as_mut() {
            node
        } else {
            panic!("Node #{index} has been released or has an invalid node index")
        }
    }
}

impl<T: Debug> Tree<T> {
    /// Creates an empty tree.
    pub fn new() -> Tree<T> {
        Tree::default()
    }
    #[allow(unused)]
    /// 追加预留（v2C C4：inline 相位按 pending 规模一次扩容，减少倍增搬迁）
    pub(crate) fn reserve_nodes(&mut self, additional: usize) {
        self.nodes.reserve(additional);
    }

    /// Creates an empty tree with storage for at least `cap` node slots.
    pub fn with_capacity(cap: usize) -> Tree<T> {
        Tree {
            nodes: Vec::with_capacity(cap),
            forks: vec![],
            cur: None,
            free_flags: Vec::new(),
        }
    }
    #[allow(unused)]
    pub fn cur(&self) -> Option<usize> {
        self.cur
    }
    /// Appends a node at the current construction position and returns its ID.
    pub fn append(&mut self, node: T) -> usize {
        let next = self.create_node_attached(node);
        // 如果当前索引存在则进行顺序追加
        if let Some(cur) = self.cur.filter(|idx| !self.is_free_node(idx)) {
            let parent = self.get_parent(cur);
            self.nodes[cur].next = link_to(next);
            self.nodes[next].prev = link_to(cur);
            self.nodes[parent].last_child = link_to(next);
        }
        // 如果当前索引不存在则意味着存在分叉，为最后一个分叉位置创建一个子节点
        else if let Some(&parent) = self.forks.last() {
            if self.nodes[parent].first_child.is_none() {
                self.nodes[parent].first_child = link_to(next)
            }
            self.nodes[next].prev = self.nodes[parent].last_child;
            // 如果前一个节点为空则补充
            if let Some(prev) = link_get(self.nodes[next].prev) {
                assert!(self.nodes[prev].next.is_none());
                self.nodes[prev].next = link_to(next)
            }
            self.nodes[parent].last_child = link_to(next);
        }
        self.cur = Some(next);
        next
    }
    /// Appends a new last child under `parent` and returns its ID.
    pub fn append_child(&mut self, parent: usize, node: T) -> usize {
        let index = self.create_node_attached(node);
        if let Some(last_child) = link_get(self.nodes[parent].last_child) {
            self.nodes[last_child].next = link_to(index);
            self.nodes[index].prev = link_to(last_child);
            self.nodes[parent].last_child = link_to(index);
        } else {
            self.nodes[parent].first_child = link_to(index);
            self.nodes[parent].last_child = link_to(index);
        }
        self.nodes[index].parent = parent;
        index
    }

    /// Replaces the value at `idx` while preserving its structural links.
    pub fn replace(&mut self, idx: usize, item: T) -> Option<TreeNode<T>> {
        if idx >= self.nodes.len() {
            return None;
        }
        let node = TreeNode {
            item: Some(item),
            parent: self.nodes[idx].parent,
            first_child: self.nodes[idx].first_child,
            last_child: self.nodes[idx].last_child,
            next: self.nodes[idx].next,
            prev: self.nodes[idx].prev,
        };
        Some(std::mem::replace(&mut self.nodes[idx], node))
    }

    /// Enters the current node's child branch during tree construction.
    pub fn push(&mut self) -> usize {
        let Some(cur_ix) = self.cur else {
            panic!("Tree::push called without current node");
        };
        self.forks.push(cur_ix);
        self.cur = link_get(self.nodes[cur_ix].first_child);
        cur_ix
    }
    /// Leaves the current construction branch and returns its parent node ID.
    pub fn pop(&mut self) -> Option<usize> {
        let index = Some(self.forks.pop()?);
        self.cur = index;
        index
    }
    /// Creates a detached node and returns its ID.
    pub fn create_node(&mut self, item: T) -> usize {
        let index = self.nodes.len();
        self.nodes.push(TreeNode {
            item: Some(item),
            parent: self.peek_up().unwrap_or(0),
            first_child: None,
            last_child: None,
            next: None,
            prev: None,
        });
        self.mark_free(index);
        index
    }

    /// 创建节点但不加入 free 集合（用于 append/append_child 内部调用，避免标记开销）
    fn create_node_attached(&mut self, item: T) -> usize {
        let index = self.nodes.len();
        self.nodes.push(TreeNode {
            item: Some(item),
            parent: self.peek_up().unwrap_or(0),
            first_child: None,
            last_child: None,
            next: None,
            prev: None,
        });
        index
    }

    #[inline]
    fn mark_free(&mut self, index: usize) {
        if self.free_flags.len() <= index {
            self.free_flags.resize(index + 1, false);
        }
        self.free_flags[index] = true;
    }
    #[inline]
    fn clear_free(&mut self, index: usize) {
        if let Some(flag) = self.free_flags.get_mut(index) {
            *flag = false;
        }
    }

    /// Returns the active construction branch, if any.
    pub fn peek_up(&self) -> Option<usize> {
        self.forks.last().copied()
    }

    /// Clears construction state and resets the current node to the root.
    pub fn reset(&mut self) {
        self.cur = if self.is_empty() { None } else { Some(0) };
        self.forks.clear();
    }

    /// Returns `true` when the tree contains no nodes below the root.
    pub fn is_empty(&self) -> bool {
        self.nodes.len() <= 1
    }

    /// Returns the number of live nodes, including the root.
    pub fn len(&self) -> usize {
        self.nodes.iter().filter(|it| it.item.is_some()).count()
    }

    /// Returns the number of allocated slots, including released slots, in O(1).
    pub fn node_slots_len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns whether an allocated slot is currently detached.
    #[inline]
    pub(crate) fn is_free(&self, index: usize) -> bool {
        self.free_flags.get(index).copied().unwrap_or(false)
    }

    /// Returns the parent node ID. The root is its own parent.
    pub fn get_parent(&self, index: usize) -> usize {
        self.nodes[index].parent
    }
    /// Attaches a detached node as the last child of `parent`.
    pub fn set_parent(&mut self, index: usize, parent: usize) {
        assert!(self.is_free_node(&index), "node must be free node");
        // #[cfg(debug_assertions)]
        // println!(
        //     "set node #{index} parse，from {} to #{parent}",
        //     self.nodes[index].parent
        // );
        self.nodes[index].parent = parent;
        self.clear_free(index);
        if let Some(last_child) = link_get(self.nodes[parent].last_child) {
            assert!(
                self.nodes[last_child].next.is_none(),
                "#{last_child} next node is invalid"
            );
            self.nodes[last_child].next = link_to(index);
            self.nodes[index].prev = link_to(last_child);
            self.nodes[parent].last_child = link_to(index);
        } else {
            let parent = &mut self.nodes[parent];
            parent.first_child = link_to(index);
            parent.last_child = link_to(index);
        }
    }
    /// Returns the first child of `index`.
    pub fn get_first_child(&self, index: usize) -> Option<usize> {
        link_get(self.nodes[index].first_child)
    }
    /// Returns the last child of `index`.
    pub fn get_last_child(&self, index: usize) -> Option<usize> {
        link_get(self.nodes[index].last_child)
    }
    /// Returns the next sibling of `index`.
    pub fn get_next(&self, index: usize) -> Option<usize> {
        link_get(self.nodes[index].next)
    }
    /// Moves `next` directly after `index`.
    ///
    /// Both nodes must have the same parent.
    pub fn set_next(&mut self, index: usize, next: usize) {
        assert_eq!(
            self.get_parent(index),
            self.get_parent(next),
            "Must have the same parent"
        );
        // 断开 next 节点关系
        self.unlink(next);
        self.nodes[next].parent = self.get_parent(index);
        // 重写 next 关系
        if let Some(prior_next) = self.get_next(index) {
            self.nodes[next].next = link_to(prior_next);
            self.nodes[prior_next].prev = link_to(next);
        };
        // 设置 next
        self.nodes[index].next = link_to(next);
        self.nodes[next].prev = link_to(index);
    }
    /// Returns the previous sibling of `index`.
    pub fn get_prev(&self, index: usize) -> Option<usize> {
        link_get(self.nodes[index].prev)
    }

    /// Moves `prev` directly before `index`.
    ///
    /// Both nodes must have the same parent.
    pub fn set_prev(&mut self, index: usize, prev: usize) {
        assert_eq!(
            self.get_parent(index),
            self.get_parent(prev),
            "Must have the same parent, left = #{index}{:?} right = #{prev}{:?}",
            self.nodes[index].item,
            self.nodes[prev].item
        );
        // 断开 prev 节点关系
        self.unlink(prev);
        self.nodes[prev].parent = self.get_parent(index);
        // 重写 prev 关系
        if let Some(prior_prev) = self.get_prev(index) {
            self.nodes[prev].prev = link_to(prior_prev);
            self.nodes[prior_prev].next = link_to(prev);
        };
        // 设置 prev
        self.nodes[index].prev = link_to(prev);
        self.nodes[prev].next = link_to(index);
    }
    /// Removes and returns a leaf node.
    ///
    /// Panics if the node still has children.
    pub fn remove(&mut self, idx: usize) -> T {
        assert!(
            self.get_first_child(idx).is_none(),
            "Expected no child nodes for index {}",
            idx
        );
        assert!(
            self.get_last_child(idx).is_none(),
            "Expected no child nodes for index {}",
            idx
        );
        self.unlink(idx);
        let item = self.nodes[idx].item.take();
        self.nodes[idx].parent = 0;
        self.nodes[idx].first_child = None;
        self.nodes[idx].last_child = None;
        self.nodes[idx].next = None;
        self.nodes[idx].prev = None;
        self.clear_free(idx);
        match item {
            Some(item) => item,
            None => panic!("Node #{idx} has been released or has an invalid node index"),
        }
    }
    /// Detaches a node from its parent and sibling chain.
    pub fn unlink(&mut self, idx: usize) {
        // 断开父节点
        let parent = self.get_parent(idx);
        match (
            link_get(self.nodes[parent].first_child) == Some(idx),
            link_get(self.nodes[parent].last_child) == Some(idx),
        ) {
            (true, true) => {
                self.nodes[parent].first_child = None;
                self.nodes[parent].last_child = None;
            }
            (true, false) => {
                self.nodes[parent].first_child = self.nodes[idx].next;
            }
            (false, true) => {
                self.nodes[parent].last_child = self.nodes[idx].prev;
            }
            (false, false) => (),
        }
        self.nodes[idx].parent = 0;
        self.mark_free(idx);
        // 断开前后节点
        if let Some(prev) = link_get(self.nodes[idx].prev) {
            self.nodes[prev].next = self.nodes[idx].next
        }
        if let Some(next) = link_get(self.nodes[idx].next) {
            self.nodes[next].prev = self.nodes[idx].prev
        }
        self.nodes[idx].next = None;
        self.nodes[idx].prev = None;
    }
    /// Returns `true` when the node is detached from the live tree.
    pub fn is_free_node(&self, idx: &usize) -> bool {
        self.free_flags.get(*idx).copied().unwrap_or(false)
    }
    /// 槽位存在且节点未被移除
    pub(crate) fn node_exists(&self, idx: usize) -> bool {
        self.nodes
            .get(idx)
            .map(|node| node.item.is_some())
            .unwrap_or(false)
    }
    #[cfg(debug_assertions)]
    pub fn print_link_info(&self, title: &str, idx: usize) {
        println!("[{title}]: ({:?})", self.nodes[idx].last_child);
        let mut item = link_get(self.nodes[idx].first_child);
        while let Some(next) = item {
            if let Some(item) = self.nodes[next].item.as_ref() {
                print!("->#{next}{item:?}");
            } else {
                print!("->#{next}<Free>");
            }
            item = link_get(self.nodes[next].next);
        }
        println!();
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            forks: Vec::new(),
            cur: None,
            free_flags: Vec::new(),
        }
    }
}
impl<T> Debug for Tree<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn debug_tree<T>(
            tree: &Tree<T>,
            cur: usize,
            indent: usize,
            f: &mut std::fmt::Formatter,
        ) -> std::fmt::Result
        where
            T: Debug,
        {
            for _ in 0..indent {
                write!(f, "  ")?;
            }
            if let Some(item) = tree.nodes[cur].item.as_ref() {
                writeln!(f, "{:?}", item)?;
            } else {
                writeln!(f, "<Free>")?;
            }
            if let Some(child_ix) = link_get(tree.nodes[cur].first_child) {
                debug_tree(tree, child_ix, indent + 1, f)?;
            }
            if let Some(next_ix) = link_get(tree.nodes[cur].next) {
                debug_tree(tree, next_ix, indent, f)?;
            }
            Ok(())
        }

        if self.nodes.len() > 1 {
            debug_tree(self, 0, 0, f)
        } else {
            write!(f, "Empty tree")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;

    /// P5 链接压缩后的槽位尺寸上限（压缩前 152 字节，`Option<usize>` 链接 ×4）。
    #[test]
    fn tree_node_slot_is_compact() {
        assert!(
            std::mem::size_of::<TreeNode<crate::node::Node>>() <= 120,
            "TreeNode<Node> = {} bytes",
            std::mem::size_of::<TreeNode<crate::node::Node>>()
        );
    }
}
