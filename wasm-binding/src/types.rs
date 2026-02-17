use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_TYPE_CONST: &'static str = r##"
/**
 * YAML 值类型，支持字符串、数字、布尔值、null 或嵌套数组
 * YAML value type, supports string, number, boolean, null or nested arrays
 */
export type YamlValue = string | number | boolean | null | readonly YamlValue[];

/**
 * Frontmatter 对象，键值对映射
 * Frontmatter object, key-value mapping
 */
export type Frontmatter = Record<string, YamlValue>;

/**
 * 文档中的位置信息
 * Location information in the document
 */
export interface Location{
    /** 行号（从 1 开始）/ Line number (1-based) */
    line: number,
    /** 列号（从 1 开始）/ Column number (1-based) */
    column: number
}

/**
 * 标签数组（无序）
 * Tags array (unsorted) - do not rely on order
 */
export type Tags = string[];

/**
 * 解析模式
 * Parse mode
 */
export type ParseMode = "full" | "frontmatter_only";

/**
 * 解析器选项配置
 * Parser options configuration
 */
export interface ParserOptions {
    /**
     * 解析策略 / Parse strategy:
     * - "full" (默认/default): 一次性解析完整文档 / parse full document in one call
     * - "frontmatter_only": 仅第一阶段（Document + FrontMatter），然后调用 continue_parse() 进行第二阶段
     *                       / phase 1 only (Document + FrontMatter), then call continue_parse() for phase 2
     */
    readonly parse_mode?: ParseMode
    /** 启用 GitHub Flavored Markdown 模式 / Enable GitHub Flavored Markdown mode */
    readonly github_flavored?: boolean
    /** 启用 GFM 扩展自动链接 / Enable GFM extended autolink */
    readonly gfm_extended_autolink?: boolean
    /** 启用 Obsidian Flavored Markdown 模式 / Enable Obsidian Flavored Markdown mode */
    readonly obsidian_flavored?: boolean
    /** 启用 MDX 组件解析（解析 HTML 时支持 JSX 语法）/ Enable MDX component parsing (support JSX syntax when parsing HTML) */
    readonly mdx_component?: boolean
    /** 启用 CJK 自动纠正 / Enable CJK autocorrect */
    readonly cjk_autocorrect?: boolean
    /** 启用智能标点转换 / Enable smart punctuation transforms */
    readonly smart_punctuation?: boolean
    /** 规范化中文标点符号 / Normalize Chinese punctuation */
    readonly normalize_chinese_punctuation?: boolean
    /** 启用 CJK 友好的分隔符规则 / Enable CJK-friendly delimiter rules */
    readonly cjk_friendly_delimiters?: boolean
    /** 输入大小限制（字节）/ Input size limit (bytes) */
    readonly max_input_bytes?: number
    /** 节点数量限制 / Node count limit */
    readonly max_nodes?: number
    /** 预配置的 CJK 名词列表 / Preconfigured CJK nouns list */
    readonly cjk_nouns?: readonly string[]
    /** 从 frontmatter 字段读取额外的 CJK 名词 / Read extra CJK nouns from frontmatter field */
    readonly cjk_nouns_from_frontmatter?: string
}
/**
 * 引用类型，用于 Obsidian 风格的内部链接
 * Reference type for Obsidian-style internal links
 */
export type Reference = {
    /** 标题引用 / Heading reference */
    readonly variant: 'heading',
    /** 标题文本 / Heading text */
    readonly value: string
} | {
    /** 多级标题引用 / Multi-level heading reference */
    readonly variant: 'multi-heading',
    /** 标题路径数组 / Heading path array */
    readonly value: readonly string[]
} | {
    /** 块 ID 引用 / Block ID reference */
    readonly variant: 'block-id',
    /** 块 ID 值 / Block ID value */
    readonly value: string
}

/**
 * AST 节点基础接口
 * Base interface for AST nodes
 */
export interface Node{
    /**
     * 块标识符，仅 block 节点有此属性
     * Block identifier, only block nodes have this property
     * 
     * @see https://help.obsidian.md/Linking+notes+and+files/Internal+links#Link+to+a+block+in+a+note
     */
    readonly id: string | undefined
    /**
     * 节点类型，决定 content 的结构
     * Node type, determines the structure of content
     */
    readonly kind: string;
    /**
     * 节点内容，可能为空，取决于节点类型
     * Node content, may be undefined depending on node type
     */
    readonly content?: unknown
    /** 节点起始位置 / Node start position */
    readonly start: Location
    /** 节点结束位置 / Node end position */
    readonly end: Location
    /**
     * 所有子节点，TextNode 没有子节点
     * All child nodes, TextNode has no children
     */
    readonly children: ReadonlyArray<AstNode>
}
/** 文档根节点 / Document root node */
export interface DocumentNode extends Node{
    readonly kind: "document"
}

/** Frontmatter 节点（YAML 元数据）/ Frontmatter node (YAML metadata) */
export interface FrontMatterNode extends Node{
    readonly kind: "frontmatter"
    readonly content: Frontmatter
}

/** 段落节点 / Paragraph node */
export interface ParagraphNode extends Node{
    readonly kind: "paragraph"
}

/** 软换行节点 / Soft break node */
export interface SoftBreakNode extends Node{
    readonly kind: "soft-break"
}

/** 硬换行节点 / Hard break node */
export interface HardBreakNode extends Node{
    readonly kind: "hard-break"
}

/** 文本节点 / Text node */
export interface TextNode extends Node{
    readonly kind: "text"
    /** 文本内容 / Text content */
    readonly content: string
}

/**
 * 嵌入节点（Obsidian 风格）
 * Embed node (Obsidian-style)
 */
export interface EmbedNode extends Node{
    readonly kind: "embed"
    readonly content: {
        /** 嵌入文件路径 / Embedded file path */
        readonly path: string,
        /** 尺寸 [宽度, 高度?] / Size [width, height?] */
        readonly size: readonly [number, number | undefined] | undefined,
        /** 引用信息 / Reference information */
        readonly reference: Reference | undefined
        /** 属性列表 / Attributes list */
        readonly attrs: readonly [name: string, value: string][] | undefined
    }
}

/**
 * 标题节点
 * Heading node
 */
export interface HeadingNode extends Node{
    readonly kind: "heading"
    readonly content: {
        /** 标题级别 / Heading level */
        readonly level: 'H1' | 'H2' | 'H3' | 'H4' | 'H5' | 'H6',
    }
}

/** 粗体节点 / Strong (bold) node */
export interface StrongNode extends Node{
    readonly kind: "strong"
}

/** 斜体节点 / Emphasis (italic) node */
export interface EmphasisNode extends Node{
    readonly kind: "emphasis"
}

/**
 * 列表节点
 * List node
 */
export interface ListNode extends Node{
    readonly kind: "list"
    readonly content: {
        /** 无序列表或任务列表 / Bullet list or task list */
        readonly variant: "bullet" | "task"
        /** 是否紧凑（无空行）/ Whether tight (no blank lines) */
        readonly tight: boolean
    } | {
        /** 有序列表 / Ordered list */
        readonly variant: "ordered"
        /** 是否紧凑（无空行）/ Whether tight (no blank lines) */
        readonly tight: boolean
        /** 起始编号 / Start number */
        readonly start: number
    }
}

/**
 * 列表项节点
 * List item node
 */
export interface ListItemNode extends Node{
    readonly kind: "list-item"
    readonly content: {
        /** 仅当列表类型为 ordered 时有值 / Only if list variant is `ordered` */
        readonly start: number | undefined
        /** 仅当列表类型为 task 时有值，' ' 表示未选中 / Only if list variant is `task`, ' ' means not checked */
        readonly task: string | undefined
    }
}

/**
 * 图片节点
 * Image node
 */
export interface ImageNode extends Node{
    readonly kind: "image"
    readonly content: {
        /** 图片 URL / Image URL */
        readonly url: string
        /** 图片标题 / Image title */
        readonly title: string | undefined
    }
}
/**
 * 链接节点
 * Link node
 */
export interface LinkNode extends Node{
    readonly kind: "link"
    readonly content: {
        /** 标准链接 / Standard link */
        readonly variant: "default"
        /** 链接 URL / Link URL */
        readonly url: string
        /** 链接标题 / Link title */
        readonly title: string | undefined
    } | {
        /** Wiki 链接（Obsidian 风格）/ Wiki link (Obsidian-style) */
        readonly variant: "wikilink"
        /** 链接路径 / Link path */
        readonly path: string
        /** 显示文本 / Display text */
        readonly text: string | undefined
        /** 引用信息 / Reference information */
        readonly reference: Reference | undefined
    } | {
        /** 脚注引用 / Footnote reference */
        readonly variant: "footnote"
        /** 脚注标签 / Footnote label */
        readonly footnote_label: string
        /** 脚注索引 / Footnote index */
        readonly index: number
        /** 引用次数 / Reference count */
        readonly ref_count: number
    }| {
        /** 脚注反向引用 / Footnote backref */
        readonly variant: "footnote-backref"
        /** 脚注标签 / Footnote label */
        readonly footnote_label: string
        /** 脚注索引 / Footnote index */
        readonly index: number
    }
}

/** 标签节点（如 #tag）/ Tag node (e.g. #tag) */
export interface TagNode extends Node{
    readonly kind: "tag"
    /** 标签名称 / Tag name */
    readonly content: string
}

/** Emoji 节点 / Emoji node */
export interface EmojiNode extends Node{
    readonly kind: "emoji"
    /** Emoji 内容 / Emoji content */
    readonly content: string
}

/** 引用块节点 / Block quote node */
export interface BlockQuoteNode extends Node{
    readonly kind: "block-quote"
}

/**
 * 代码节点
 * Code node
 */
export interface CodeNode extends Node{
    readonly kind: "code"
    readonly content: {
        /** 行内代码 / Inline code */
        readonly variant: "inline"
    } | {
        /** 缩进代码块 / Indented code block */
        readonly variant: "indent"
    } | {
        /** 围栏代码块 / Fenced code block */
        readonly variant: "fenced"
        /** 编程语言 / Programming language */
        readonly language: string | undefined
    }
}
/**
 * 表格节点
 * Table node
 */
export interface TableNode extends Node{
    readonly kind: "table"
    readonly content: {
        /** 列数 / Column count */
        readonly column: number
        /** 列对齐方式 / Column alignments */
        readonly alignments: readonly ("left" | "center" | "right")[]
    }
}

/** 表格头部节点 / Table head node */
export interface TableHeadNode extends Node{
    readonly kind: "table-head"
}

/** 表格头部列节点 / Table head column node */
export interface TableHeadColNode extends Node{
    readonly kind: "table-head-col"
}

/** 表格主体节点 / Table body node */
export interface TableBodyNode extends Node{
    readonly kind: "table-body"
}

/** 表格行节点 / Table row node */
export interface TableRowNode extends Node{
    readonly kind: "table-row"
}

/** 表格数据列节点 / Table data column node */
export interface TableDataColNode extends Node{
    readonly kind: "table-data-col"
}

/** 删除线节点 / Strikethrough node */
export interface StrikethroughNode extends Node{
    readonly kind: "strikethrough"
}

/** 高亮节点（Obsidian 风格）/ Highlighting node (Obsidian-style) */
export interface HighlightingNode extends Node{
    readonly kind: "highlighting"
}

/** 主题分隔线节点 / Thematic break node */
export interface ThematicBreakNode extends Node{
    readonly kind: "thematic-break"
}

/**
 * 脚注定义节点
 * Footnote definition node
 */
export interface FootnoteNode extends Node{
    readonly kind: "footnote"
    readonly content: {
        /** 脚注标签 / Footnote label */
        readonly label: string
        /** 引用次数 / Reference count */
        readonly ref_count: number
    }
}

/** 脚注列表节点 / Footnote list node */
export interface FootnoteListNode extends Node{
    readonly kind: "footnote-list"
}

/**
 * 数学公式节点
 * Math node
 */
export interface MathNode extends Node{
    readonly kind: "math"
    readonly content: {
        /** 行内或块级公式 / Inline or block math */
        readonly variant: "inline" | "block"
    }
}

/**
 * 标注节点（Obsidian 风格）
 * Callout node (Obsidian-style)
 */
export interface CalloutNode extends Node{
    readonly kind: "callout"
    readonly content: {
        /** 标注类型 / Callout type */
        readonly type: "note" | "success" | "warning" | "failure" | "error" | string
        /** 标注标题 / Callout title */
        readonly title: string | undefined
        /** 是否可折叠 / Whether foldable */
        readonly foldable: boolean | undefined
    }
}

/**
 * HTML 属性值类型
 * HTML property value type
 */
export type PropValue = { literal: string } | { expr: string };

/**
 * HTML 节点
 * HTML node
 */
export interface HtmlNode extends Node{
    readonly kind: "html"
    readonly content: {
        /** 行内或块级 HTML / Inline or block HTML */
        readonly variant: "inline" | "block"
        /** HTML 注释或表达式类型 / HTML comment or expression type */
        readonly type: "type2" | "type3" | "type4" | "type5" | "js_comment" | "js_expression"
    } | {
        /** 行内或块级 HTML / Inline or block HTML */
        readonly variant: "inline" | "block"
        /** HTML 标签或组件类型 / HTML tag or component type */
        readonly type: "type1" | "type6" | "type7" | "component"
        /** 标签名称 / Tag name */
        readonly name: string
        /** 属性列表 / Properties list */
        readonly props: readonly [name: string, value: PropValue][] | undefined
        /** 标签标志 / Tag flag */
        readonly flag: "begin" | "end" | "full" | "self-close"
    }
}

/**
 * AST 节点联合类型
 * AST node union type
 */
export type AstNode = DocumentNode | FrontMatterNode | ParagraphNode | SoftBreakNode | HardBreakNode
    | TextNode | EmbedNode | HeadingNode | StrongNode | EmphasisNode | ListNode | ListItemNode
    | ImageNode | LinkNode | TagNode | EmojiNode | BlockQuoteNode | CodeNode | TableNode
    | TableHeadNode | TableHeadColNode | TableBodyNode | TableRowNode | TableDataColNode
    | StrikethroughNode | HighlightingNode | ThematicBreakNode | FootnoteNode | FootnoteListNode
    | MathNode | CalloutNode | HtmlNode
"##;
