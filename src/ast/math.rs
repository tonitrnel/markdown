use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "variant", rename_all = "kebab-case")]
pub enum Math {
    Inline(InlineMath),
    Block(BlockMath),
}
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct InlineMath {}
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct BlockMath {}
