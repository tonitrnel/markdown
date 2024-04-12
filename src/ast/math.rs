#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Math {
    Inline(InlineMath),
    Block(BlockMath),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineMath{}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockMath{}