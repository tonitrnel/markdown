# TODO

- [x] [渐进式迭代 Wayfinder 决策地图（01–07 已决，08 待 P7 证据）](.scratch/markdown-parser-incremental-iteration/map.md)
- [x] [v2A 全部完成（09–18 resolved；2x 达成、同类打平；证据见 spec 与 bench/results）](.scratch/markdown-parser-incremental-iteration/spec.md)
- [ ] [v2B 完成（tickets 19–21，三终点全达成：同类反超、OFM ≤5%、分配 −60%；1.42x pulldown）。v2C 进行中（tickets 22–26）：C2+C5 ✓、C3 ✓（corpus 会话 1,875→1,092 µs 终点命中）、C4 ✓（block 会话 −8%、realloc −96%）；C1 ✓（LineHead 保留，门禁未达如实记档：35.1% 主体为容器下降/行循环，域外）。v2C 收官：corpus 会话 1,875→1,012 µs ✓、_data 会话 728→596 µs（≤500 未达记档）、全量净 −2~3%。WASM 交付面（tickets 27/28/30 ✓）：目标寻址过 binding 0.98 ms（16.7x）、SIMD 固化；W1 ✓ 直写序列化 tree_json（全树 2.0x/2.2x，corpus 反超 markdown-it；≤8ms 门差 3.7% 记档）。WASM 交付面全线收官（27-30）](.scratch/markdown-parser-incremental-iteration/issues/19-m1-link-escape-text-lowering.md)
- [x] [相邻脚注引用吞并 bug（已修复 2026-07-26）](.scratch/footnote-adjacent-refs/issues/01-adjacent-footnote-refs-swallowed.md)
- [ ] [渐进式解析器迭代计划 r2（门槛与阶段细节）](docs/plans/2026-07-26-markdown-parser-incremental-iteration-r2_cn.md)
- [x] [脚注编号文档顺序 bug（B1，已修复 2026-07-26）](.scratch/footnote-numbering-order/issues/01-footnote-numbering-follows-hashmap-order.md)
- [ ] [选择性 Inline 语义](docs/specs/2026-07-19-selective-inline-events-design.md)
- [ ] [性能架构证据与挑战目标](docs/specs/2026-07-20-markdown-parser-performance-architecture-design.md)
