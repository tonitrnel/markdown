use std::fmt::Debug;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct TreeNode<T> {
    #[allow(unused)]
    pub item: T,
    parent: usize,
    child: Option<usize>,
    next: Option<usize>,
}

impl<T> TreeNode<T> {
    pub(crate) fn get_parent(&self) -> usize {
        self.parent
    }
}

impl<T: PartialEq> PartialEq<T> for TreeNode<T> {
    fn eq(&self, other: &T) -> bool {
        self.item.eq(other)
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
}
impl<T> Index<usize> for Tree<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes.index(index).item
    }
}
impl<T> IndexMut<usize> for Tree<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes.index_mut(index).item
    }
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree {
            nodes: Vec::new(),
            forks: Vec::new(),
            cur: None,
        }
    }
    #[allow(unused)]
    pub fn with_capacity(cap: usize) -> Tree<T> {
        Tree {
            nodes: Vec::with_capacity(cap),
            forks: vec![],
            cur: None,
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
        let index = self.create_node(node);
        // 如果当前索引存在则进行顺序追加
        if let Some(idx) = self.cur {
            self.nodes[idx].next = Some(index);
        }
        // 如果当前索引不存在则意味着存在分叉，为最后一个分叉位置创建一个子节点
        else if let Some(&parent) = self.forks.last() {
            self.nodes[parent].child = Some(index);
        }
        self.cur = Some(index);
        index
    }
    // pub fn batch_append(&mut self, nodes: Vec<T>) {
    //     for node in nodes {
    //         self.append(node);
    //     }
    // }
    pub fn replace(&mut self, idx: usize, item: T) -> Option<TreeNode<T>> {
        if idx >= self.nodes.len() {
            return None;
        }
        let node = TreeNode {
            item,
            parent: self.nodes[idx].parent,
            child: self.nodes[idx].child,
            next: self.nodes[idx].next,
        };
        return Some(std::mem::replace(&mut self.nodes[idx], node));
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
        self.cur = self.nodes[cur_ix].child;
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
            item,
            parent: self.peek_up().unwrap_or(0),
            child: None,
            next: None,
        });
        index
    }

    /// 将一个节点下所有子节点变成同级节点
    // pub fn flatten(&mut self, idx: usize) {
    //     if let Some(node) = self.nodes.get_mut(idx) {
    //         let next_idx = node.next;
    //         if let Some(child_idx) = node.child {
    //             node.next = Some(child_idx);
    //             node.child = None;
    //             if let Some(next_idx) = next_idx {
    //                 let mut cur = child_idx;
    //                 loop {
    //                     if let Some(target) = self.nodes.get_mut(cur) {
    //                         if let Some(next_cur) = target.next {
    //                             cur = next_cur;
    //                         } else {
    //                             target.next = Some(next_idx);
    //                         }
    //                     }
    //                 }
    //             }
    //         };
    //         self.forks.retain(|&x| x == idx);
    //     }
    // }
    pub fn create_scope<F>(&mut self, node: T, f: F)
    where
        F: FnOnce(&mut Tree<T>),
    {
        self.append(node);
        self.push();
        f(self);
        self.pop();
    }

    #[allow(unused)]
    pub fn move_next_sibling(&mut self, index: usize) -> Option<usize> {
        self.cur = self.nodes[index].next;
        self.cur
    }

    ///查看父节点
    #[allow(unused)]
    pub fn peek_up(&self) -> Option<usize> {
        self.forks.last().copied()
    }

    #[allow(unused)]
    pub fn reset(&mut self) {
        self.cur = if self.is_empty() { None } else { Some(0) };
        self.forks.clear();
    }

    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.nodes.len() <= 1
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_parent(&self, index: usize) -> usize {
        self.nodes[index].parent
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
            writeln!(f, "{:?}", &tree.nodes[cur].item)?;
            if let Some(child_ix) = tree.nodes[cur].child {
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
