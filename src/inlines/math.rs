#![allow(unused)]
use crate::ast::math;
use crate::inlines::ProcessCtx;

impl math::InlineMath {
    pub(super) fn parse(ProcessCtx { line, parser, id,.. }: &mut ProcessCtx) -> bool{
        todo!()
    }
}
impl math::BlockMath{
    pub(super) fn parse(ProcessCtx { line, parser, id,.. }: &mut ProcessCtx) -> bool{
        todo!()
    }
}