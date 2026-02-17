use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_TYPE_CONST: &'static str = r##"
export type YamlValue = string | number | boolean | null | readonly YamlValue[];
export type Frontmatter = Record<string, YamlValue>;
export interface Location{
    line: number,
    column: number
}
// tags getter returns an unsorted array; do not rely on order.
export type Tags = string[];
export type ParseMode = "full" | "frontmatter_only";
export interface ParserOptions {
    // Parse strategy:
    // - "full" (default): parse full document in one call.
    // - "frontmatter_only": phase 1 only (Document + FrontMatter), then call continue_parse() for phase 2.
    readonly parse_mode?: ParseMode
    readonly github_flavored?: boolean
    readonly gfm_extended_autolink?: boolean
    readonly obsidian_flavored?: boolean
    readonly mdx_component?: boolean
    readonly cjk_autocorrect?: boolean
    readonly smart_punctuation?: boolean
    readonly normalize_chinese_punctuation?: boolean
    readonly cjk_friendly_delimiters?: boolean
    readonly max_input_bytes?: number
    readonly max_nodes?: number
    readonly cjk_nouns?: readonly string[]
    readonly cjk_nouns_from_frontmatter?: string
}
export type Reference = {
    readonly variant: 'heading',
    readonly value: string
} | {
    readonly variant: 'multi-heading',
    readonly value: readonly string[]
} | {
    readonly variant: 'block-id',
    readonly value: string
}
export interface Node{
    // block 标识符, 只有 block 节点会有
    //
    // see https://help.obsidian.md/Linking+notes+and+files/Internal+links#Link+to+a+block+in+a+note
    readonly id: string | undefined
    // 该节点的种类，根据该值确定 content 的结构
    readonly kind: string;
    // 节点的内容，可能没有，取决于节点的 kind
    readonly content?: unknown
    // 节点起始位置
    readonly start: Location
    // 节点结束位置
    readonly end: Location
    // 所有的子节点，TextNode 没有
    readonly children: ReadonlyArray<AstNode>
}
export interface DocumentNode extends Node{
    readonly kind: "document"
}
export interface FrontMatterNode extends Node{
    readonly kind: "frontmatter"
    readonly content: Frontmatter
}
export interface ParagraphNode extends Node{
    readonly kind: "paragraph"
}
export interface SoftBreakNode extends Node{
    readonly kind: "soft-break"
}
export interface HardBreakNode extends Node{
    readonly kind: "hard-break"
}
export interface TextNode extends Node{
    readonly kind: "text"
    readonly content: string
}
export interface EmbedNode extends Node{
    readonly kind: "embed"
    readonly content: {
        readonly path: string,
        readonly size: readonly [number, number | undefined] | undefined,
        readonly reference: Reference | undefined
        readonly attrs: readonly [name: string, value: string][] | undefined
    }
}
export interface HeadingNode extends Node{
    readonly kind: "heading"
    readonly content: {
        readonly level: 'H1' | 'H2' | 'H3' | 'H4' | 'H5' | 'H6',
    }
}
export interface StrongNode extends Node{
    readonly kind: "strong"
}
export interface EmphasisNode extends Node{
    readonly kind: "emphasis"
}
export interface ListNode extends Node{
    readonly kind: "list"
    readonly content: {
        readonly variant: "bullet" | "task"
        readonly tight: boolean
    } | {
        readonly variant: "ordered"
        readonly tight: boolean
        readonly start: number
    }
}
export interface ListItemNode extends Node{
    readonly kind: "list-item"
    readonly content: {
        // only if list variant is `ordered`
        readonly start: number | undefined
        // only if list variant is `task`, if value is ' ' then not `checked`
        readonly task: string | undefined
    }
}
export interface ImageNode extends Node{
    readonly kind: "image"
    readonly content: {
        readonly url: string
        readonly title: string | undefined
    }
}
export interface LinkNode extends Node{
    readonly kind: "link"
    readonly content: {
        readonly variant: "default"
        readonly url: string
        readonly title: string | undefined
    } | {
        readonly variant: "wikilink"
        readonly path: string
        readonly text: string | undefined
        readonly reference: Reference | undefined
    } | {
        readonly variant: "footnote"
        readonly footnote_label: string
        readonly index: number
        readonly ref_count: number
    }| {
        readonly variant: "footnote-backref"
        readonly footnote_label: string
        readonly index: number
    }
}
export interface TagNode extends Node{
    readonly kind: "tag"
    readonly content: string
}
export interface EmojiNode extends Node{
    readonly kind: "emoji"
    readonly content: string
}
export interface BlockQuoteNode extends Node{
    readonly kind: "block-quote"
}
export interface CodeNode extends Node{
    readonly kind: "code"
    readonly content: {
        readonly variant: "inline"
    } | {
        readonly variant: "indent"
    } | {
        readonly variant: "fenced"
        readonly language: string | undefined
    }
}
export interface TableNode extends Node{
    readonly kind: "table"
    readonly content: {
        readonly column: number
        readonly alignments: readonly ("left" | "center" | "right")[]
    }
}
export interface TableHeadNode extends Node{
    readonly kind: "table-head"
}
export interface TableHeadColNode extends Node{
    readonly kind: "table-head-col"
}
export interface TableBodyNode extends Node{
    readonly kind: "table-body"
}
export interface TableRowNode extends Node{
    readonly kind: "table-row"
}
export interface TableDataColNode extends Node{
    readonly kind: "table-data-col"
}
export interface StrikethroughNode extends Node{
    readonly kind: "strikethrough"
}
export interface HighlightingNode extends Node{
    readonly kind: "highlighting"
}
export interface ThematicBreakNode extends Node{
    readonly kind: "thematic-break"
}
export interface FootnoteNode extends Node{
    readonly kind: "footnote"
    readonly content: {
        readonly label: string
        readonly ref_count: number
    }
}
export interface FootnoteListNode extends Node{
    readonly kind: "footnote-list"
}
export interface MathNode extends Node{
    readonly kind: "math"
    readonly content: {
        readonly variant: "inline" | "block"
    }
}
export interface CalloutNode extends Node{
    readonly kind: "callout"
    readonly content: {
        readonly type: "note" | "success" | "warning" | "failure" | "error" | string
        readonly title: string | undefined
        readonly foldable: boolean | undefined
    }
}
export type PropValue = { literal: string } | { expr: string };
export interface HtmlNode extends Node{
    readonly kind: "html"
    readonly content: {
        readonly variant: "inline" | "block"
        readonly type: "type2" | "type3" | "type4" | "type5" | "js_comment" | "js_expression"
    } | {
        readonly variant: "inline" | "block"
        readonly type: "type1" | "type6" | "type7" | "component"
        readonly name: string
        readonly props: readonly [name: string, value: PropValue][] | undefined
        readonly flag: "begin" | "end" | "full" | "self-close"
    }
}
export type AstNode = DocumentNode | FrontMatterNode | ParagraphNode | SoftBreakNode | HardBreakNode
    | TextNode | EmbedNode | HeadingNode | StrongNode | EmphasisNode | ListNode | ListItemNode
    | ImageNode | LinkNode | TagNode | EmojiNode | BlockQuoteNode | CodeNode | TableNode
    | TableHeadNode | TableHeadColNode | TableBodyNode | TableRowNode | TableDataColNode
    | StrikethroughNode | HighlightingNode | ThematicBreakNode | FootnoteNode | FootnoteListNode
    | MathNode | CalloutNode | HtmlNode
"##;
