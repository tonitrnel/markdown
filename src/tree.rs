use std::collections::HashSet;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct TreeNode<T> {
    // impl `mem::take`
    pub item: Option<T>,
    parent: usize,
    first_child: Option<usize>,
    last_child: Option<usize>,
    next: Option<usize>,
    prev: Option<usize>,
}

impl<T: PartialEq> PartialEq<T> for TreeNode<T> {
    fn eq(&self, other: &T) -> bool {
        self.item.as_ref().unwrap().eq(other)
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
pub struct Tree<T> {
    /// 存储所有节点
    nodes: Vec<TreeNode<T>>,
    /// 存储已打开的分支分叉点的索引
    forks: Vec<usize>,
    /// 存储当前索引，它可能在树主干上，也可能在树分支上或者没有
    cur: Option<usize>,
    /// 所有 free 节点的索引
    frees: HashSet<usize>,
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

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree::default()
    }
    #[allow(unused)]
    pub fn with_capacity(cap: usize) -> Tree<T> {
        Tree {
            nodes: Vec::with_capacity(cap),
            forks: vec![],
            cur: None,
            frees: HashSet::new(),
        }
    }
    #[allow(unused)]
    pub fn cur(&self) -> Option<usize> {
        self.cur
    }
    /// 在当前树的末尾添加一个新节点，并返回该节点的索引。
    ///
    /// 如果当前索引存在，则将新节点追加到当前索引节点的子节点列表的末尾。
    ///
    /// 如果当前索引不存在，意味着当前存在一个分叉点，则将新节点添加为最后一个分叉点的子节点。
    ///
    /// 参数：
    /// - `node`: 要添加的节点的数据。
    ///
    /// 返回值：
    /// 返回新节点在树中的索引。
    pub fn append(&mut self, node: T) -> usize {
        let next = self.create_node(node);
        self.frees.remove(&next);
        // 如果当前索引存在则进行顺序追加
        if let Some(cur) = self.cur.filter(|idx| !self.is_free_node(idx)) {
            let parent = self.get_parent(cur);
            self.nodes[cur].next = Some(next);
            self.nodes[next].prev = Some(cur);
            self.nodes[parent].last_child = Some(next);
        }
        // 如果当前索引不存在则意味着存在分叉，为最后一个分叉位置创建一个子节点
        else if let Some(&parent) = self.forks.last() {
            if self.nodes[parent].first_child.is_none() {
                self.nodes[parent].first_child = Some(next)
            }
            self.nodes[next].prev = self.nodes[parent].last_child;
            self.nodes[parent].last_child = Some(next);
        }
        self.cur = Some(next);
        next
    }
    pub fn append_child(&mut self, parent: usize, node: T) -> usize {
        let index = self.create_node(node);
        self.frees.remove(&index);
        if let Some(last_child) = self.nodes[parent].last_child {
            self.nodes[last_child].next = Some(index);
            self.nodes[index].prev = Some(last_child);
            self.nodes[parent].last_child = Some(index);
        } else {
            self.nodes[parent].first_child = Some(index);
            self.nodes[parent].last_child = Some(index);
        }
        self.nodes[index].parent = parent;
        index
    }

    /// 替换指定位置的节点为传入的节点
    ///
    /// 返回值:
    /// 被替换的节点
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

    /// 进入当前索引所在的分支，并返回当前分叉点的索引。
    ///
    /// 将当前索引的值添加到分叉列表中。
    ///
    /// 如果当前索引不存在子节点，则在分叉点创建一个新的子节点，并将其设置为当前索引。
    ///
    /// 返回值：
    /// 返回分叉点的索引。
    pub fn push(&mut self) -> usize {
        let cur_ix = self.cur.unwrap();
        self.forks.push(cur_ix);
        self.cur = self.nodes[cur_ix].first_child;
        cur_ix
    }
    /// 退出当前分支，并返回退出后的当前节点索引。
    ///
    /// 弹出分支列表中最后一个元素，并将其设置为当前节点的索引。
    ///
    /// 返回值：
    /// 如果成功退出了分支节点，则返回一个 `Some` 包含当前节点索引的值的枚举值；
    /// 如果分支列表为空，则返回一个 `None` 值。
    pub fn pop(&mut self) -> Option<usize> {
        let index = Some(self.forks.pop()?);
        self.cur = index;
        index
    }
    /// 创建一个节点，将该节点添加到树节点列表中后并返回该节点在树节点列表的索引
    ///
    /// 返回值：
    /// 在树节点列表上的索引
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
        self.frees.insert(index);
        index
    }

    /// 查看当前节点的父节点的 ID
    pub fn peek_up(&self) -> Option<usize> {
        self.forks.last().copied()
    }

    /// 清空分叉，这将丢失“当前节点”的指针，清空后当前的指针将指向根节点
    pub fn reset(&mut self) {
        self.cur = if self.is_empty() { None } else { Some(0) };
        self.forks.clear();
    }

    /// 节点数量是否为空
    ///
    /// 类似：`len() == 0`
    pub fn is_empty(&self) -> bool {
        self.nodes.len() <= 1
    }

    /// 获取节点数量
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// 获取上级节点的位置
    ///
    /// 注：查询节点为 Root 时会返回其自身
    pub fn get_parent(&self, index: usize) -> usize {
        self.nodes[index].parent
    }
    /// 设置节点的父级节点
    ///
    /// 注：这会将该节点添加至父节点的 `last_child`
    pub fn set_parent(&mut self, index: usize, parent: usize) {
        assert!(self.frees.contains(&index), "node must be free node");
        self.nodes[index].parent = parent;
        self.frees.remove(&index);
        if let Some(last_child) = self.nodes[parent].last_child {
            assert!(
                self.nodes[last_child].next.is_none(),
                "#{last_child} next node #{} is invalid",
                self.nodes[last_child].next.unwrap()
            );
            self.nodes[last_child].next = Some(index);
            self.nodes[index].prev = Some(last_child);
            self.nodes[parent].last_child = Some(index);
        } else {
            let parent = &mut self.nodes[parent];
            parent.first_child = Some(index);
            parent.last_child = Some(index);
        }
    }
    pub fn get_first_child(&self, index: usize) -> Option<usize> {
        self.nodes[index].first_child
    }
    pub fn get_last_child(&self, index: usize) -> Option<usize> {
        self.nodes[index].last_child
    }
    pub fn get_next(&self, index: usize) -> Option<usize> {
        self.nodes[index].next
    }
    /// 设置目标节点的一个后一个节点为指定节点
    ///
    /// 注：两个节点必需是同一个父节点，否则会 panic
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
            self.nodes[next].next = Some(prior_next);
            self.nodes[prior_next].prev = Some(next);
        };
        // 设置 next
        self.nodes[index].next = Some(next);
        self.nodes[next].prev = Some(index);
    }
    pub fn get_prev(&self, index: usize) -> Option<usize> {
        self.nodes[index].prev
    }

    /// 设置目标节点的一个前一个节点为指定节点
    ///
    /// 注：两个节点必需是同一个父节点，否则会 panic
    pub fn set_prev(&mut self, index: usize, prev: usize) {
        assert_eq!(
            self.get_parent(index),
            self.get_parent(prev),
            "Must have the same parent"
        );
        // 断开 prev 节点关系
        self.unlink(prev);
        self.nodes[prev].parent = self.get_parent(index);
        // 重写 prev 关系
        if let Some(prior_prev) = self.get_prev(index) {
            self.nodes[prev].prev = Some(prior_prev);
            self.nodes[prior_prev].next = Some(prev);
        };
        // 设置 prev
        self.nodes[index].prev = Some(prev);
        self.nodes[prev].next = Some(index);
    }
    /// 移除子节点
    ///
    /// 注：如果存在子节点将会触发 panic
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
        let node = std::mem::take(&mut self.nodes[idx]);
        self.frees.remove(&idx);
        node.item.unwrap()
    }
    /// 断掉节点的前后关系和父级关系，使该节点成为一个 `free` 节点
    ///
    /// 注：如果该节点存在字节点，那么所有子节点将会跟随该节点成为 `free` 节点
    pub fn unlink(&mut self, idx: usize) {
        // 断开父节点
        let parent = self.get_parent(idx);
        match (
            self.nodes[parent].first_child == Some(idx),
            self.nodes[parent].last_child == Some(idx),
        ) {
            (true, true) => {
                self.nodes[parent].first_child = None;
                self.nodes[parent].last_child = None;
            }
            (true, false) => {
                self.nodes[parent].first_child = self.get_next(idx);
            }
            (false, true) => {
                self.nodes[parent].last_child = self.get_prev(idx);
            }
            (false, false) => (),
        }
        self.nodes[idx].parent = 0;
        self.frees.insert(idx);
        // 断开前后节点
        if let Some(prev) = self.nodes[idx].prev {
            self.nodes[prev].next = self.nodes[idx].next
        }
        if let Some(next) = self.nodes[idx].next {
            self.nodes[next].prev = self.nodes[idx].prev
        }
        self.nodes[idx].next = None;
        self.nodes[idx].prev = None;
    }
    pub fn is_free_node(&self, idx: &usize) -> bool {
        self.frees.contains(idx)
    }
    // pub fn print_link_info(&self, title: &str, idx: usize) {
    //     println!("[{title}]: ({:?})", self.nodes[idx].last_child);
    //     let mut item = self.nodes[idx].first_child;
    //     while let Some(next) = item {
    //         if let Some(item) = self.nodes[next].item.as_ref() {
    //             print!("->#{next}{item:?}");
    //         } else {
    //             print!("->#{next}<Free>");
    //         }
    //         item = self.nodes[next].next;
    //     }
    //     println!();
    // }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            forks: Vec::new(),
            cur: None,
            frees: HashSet::new(),
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
            writeln!(f, "{:?}", &tree.nodes[cur].item.as_ref().unwrap())?;
            if let Some(child_ix) = tree.nodes[cur].first_child {
                debug_tree(tree, child_ix, indent + 1, f)?;
            }
            if let Some(next_ix) = tree.nodes[cur].next {
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
