use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use std::borrow::Cow;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "variant")]
pub enum Html {
    Block(HtmlType),
    Inline(HtmlType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Flag {
    Begin,
    End,
    // 如果在同一个容器扫描到结束标志则标记这个 HTML 为完整
    Full,
    SelfClose,
}
impl Serialize for Flag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            Flag::Begin => "begin",
            Flag::End => "end",
            Flag::Full => "full",
            Flag::SelfClose => "self-close",
        };
        serializer.serialize_str(value)
    }
}

const DISALLOWED_TAG_NAMES: [&str; 9] = [
    "title",
    "textarea",
    "style",
    "xmp",
    "iframe",
    "noembed",
    "noframes",
    "script",
    "plaintext",
];
impl Html {
    pub fn is_disallowed_raw_html(&self) -> bool {
        let typ = match self {
            Html::Block(t) => t,
            Html::Inline(t) => t,
        };
        match typ {
            HtmlType::RawTextContainer(e, _)
            | HtmlType::CanonicalBlockTag(e, _)
            | HtmlType::GenericTag(e, _)
            | HtmlType::Component(e, _) => DISALLOWED_TAG_NAMES
                .iter()
                .any(|it| e.name.eq_ignore_ascii_case(it)),
            _ => false,
        }
    }
    pub(crate) fn set_flag_is_full(&mut self) {
        let _type = match self {
            Html::Block(t) | Html::Inline(t) => t,
        };
        match _type {
            HtmlType::RawTextContainer(_, flag)
            | HtmlType::CanonicalBlockTag(_, flag)
            | HtmlType::GenericTag(_, flag)
            | HtmlType::Component(_, flag) => *flag = Flag::Full,
            _ => (),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlType {
    /// 用途：处理 `pre/script/style/textarea` 这类 raw-text 容器标签。
    /// 对应：CommonMark HTML block type 1。
    ///
    /// - Start condition: line begins with the string`<pre`,`<script`,`<style`, or`<textarea`(case-insensitive), followed by a space, a tab, the string`>`, or the end of the line.  
    /// - End condition: line contains an end tag`</pre>`,`</script>`,`</style>`, or`</textarea>`(case-insensitive; it need not match the start tag).
    RawTextContainer(Element, Flag),

    /// 用途：处理 HTML 注释块。
    /// 对应：CommonMark HTML block type 2。
    ///
    /// - Start condition: line begins with the string `<!--`.
    /// - End condition  : line contains the string `-->`.
    HtmlComment,

    /// 用途：处理 processing instruction（如 `<? ... ?>`）。
    /// 对应：CommonMark HTML block type 3。
    ///
    /// - Start condition: line begins with the string `<?`.
    /// - End condition  : line contains the string `?>`.
    ProcessingInstruction,

    /// 用途：处理声明类标签（如 `<!DOCTYPE ...>`）。
    /// 对应：CommonMark HTML block type 4。
    ///
    /// - Start condition: line begins with the string `<!` followed by an ASCII letter.
    /// - End condition  : line contains the character `>`.
    Declaration,

    /// 用途：处理 CDATA 区块。
    /// 对应：CommonMark HTML block type 5。
    ///
    /// - Start condition: line begins with the string `<![CDATA[`.
    /// - End condition  : line contains the string `]]>`.
    CDataSection,

    /// 用途：处理 CommonMark 规定的“块级 HTML 标签白名单”（规范内 canonical 列表）。
    /// 对应：CommonMark HTML block type 6。
    /// 语义：支持 `Begin/End/Full/SelfClose`，用于在块级容器中做配对与闭合管理。
    ///
    /// - Start condition: line begins with the string `<` or `</` followed by one of the strings (case-insensitive):  
    ///     `address`,`article`,`aside`,`base`,
    ///     `basefont`,`blockquote`,`body`,
    ///     `caption`,`center`,`col`,`colgroup`,
    ///     `dd`,`details`,`dialog`,`dir`,`div`,`dl`,`dt`,
    ///     `fieldset`,`figcaption`,`figure`,`footer`,`form`,`frame`,`frameset`,
    ///     `h1`,`h2`,`h3`,`h4`,`h5`,`h6`,`head`,`header`,`hr`,`html`,
    ///     `iframe`,
    ///     `legend`,`li`,`link`,
    ///     `main`,`menu`,`menuitem`,
    ///     `nav`,`noframes`,
    ///     `ol`,`optgroup`,`option`,
    ///     `p`,`param`,
    ///     `search`,`section`,`summary`,
    ///     `table`,`tbody`,`td`,`tfoot`,`th`,`thead`,`title`,`tr`,`track`,
    ///     `ul`
    ///     followed by a space, a tab, the end of the line, the string`>`, or the string`/>`.
    /// - End condition: line is followed by a blank line.
    CanonicalBlockTag(Element, Flag),

    /// 用途：处理不在 type6 白名单里的其他标签。
    /// 对应：CommonMark HTML block type 7。
    /// 语义：?
    /// - Start condition: line begins with a complete open tag (with any tag name other than `pre`, `script`, `style`, or `textarea`) or a complete closing tag, followed by zero or more spaces and tabs, followed by the end of the line.
    /// - End condition:
    ///    1. line is followed by a blank line.
    ///    2. find the closing tag that matches(custom rule, non cfm spec).
    GenericTag(Element, Flag),
    /// 用途: 自定义组件标签
    /// 对应: JSX 组件, 扩展了“同容器配对闭合”规则
    /// 语义：常用于最小 MDX 场景（例如 `<Button>...</Button>`）的 Begin/Full 标记。
    Component(Element, Flag),
    /// 用途: JS 风格的注释（`{/* ... */}`）
    JSComment(String),
    /// 用途: JS 表达式（包含类型、函数字面量、对象字面量等）
    JSExpression(String),
}

impl Serialize for HtmlType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut obj = serializer.serialize_map(Some(4))?;
        match self {
            HtmlType::RawTextContainer(..) => obj.serialize_entry("type", "type1")?,
            HtmlType::HtmlComment => obj.serialize_entry("type", "type2")?,
            HtmlType::ProcessingInstruction => obj.serialize_entry("type", "type3")?,
            HtmlType::Declaration => obj.serialize_entry("type", "type4")?,
            HtmlType::CDataSection => obj.serialize_entry("type", "type5")?,
            HtmlType::CanonicalBlockTag(..) => obj.serialize_entry("type", "type6")?,
            HtmlType::GenericTag(..) => obj.serialize_entry("type", "type7")?,
            HtmlType::Component(..) => obj.serialize_entry("type", "component")?,
            HtmlType::JSComment(..) => obj.serialize_entry("type", "js_comment")?,
            HtmlType::JSExpression(..) => obj.serialize_entry("type", "js_expression")?,
        }
        match self {
            HtmlType::RawTextContainer(element, flag)
            | HtmlType::CanonicalBlockTag(element, flag)
            | HtmlType::GenericTag(element, flag)
            | HtmlType::Component(element, flag) => {
                obj.serialize_entry("name", &element.name)?;
                obj.serialize_entry("props", &element.props)?;
                obj.serialize_entry("flag", flag)?;
            }
            HtmlType::JSComment(value) | HtmlType::JSExpression(value) => {
                obj.serialize_entry("value", value)?;
            }
            _ => (),
        }
        obj.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    pub name: String,
    pub props: Option<Vec<(String, PropValue)>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PropValue {
    Literal(String),
    Expr(String),
}

impl PropValue {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Literal(s) | Self::Expr(s) => s,
        }
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(_))
    }
    pub fn is_expression(&self) -> bool {
        matches!(self, Self::Expr(_))
    }

    pub fn literal(&self) -> Option<&str> {
        if let Self::Literal(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn expression(&self) -> Option<&str> {
        if let Self::Expr(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }
}

impl AsRef<str> for PropValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PropValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Element {
    pub(crate) fn new(name: impl AsRef<str>) -> Self {
        Self::new_with_props(name, None)
    }
    pub(crate) fn new_with_props(
        name: impl AsRef<str>,
        props: Option<Vec<(String, Cow<str>)>>,
    ) -> Self {
        Self {
            name: name.as_ref().to_string(),
            props: props.filter(|it| !it.is_empty()).map(|props| {
                props
                    .into_iter()
                    .map(|(n, v)| (n, PropValue::Literal(v.to_string())))
                    .collect()
            }),
        }
    }
    #[cfg_attr(not(test), cfg(feature = "html"))]
    pub(crate) fn attr_str(&self) -> String {
        let mut str = String::new();
        if let Some(props) = &self.props {
            for (name, value) in props.iter() {
                if value.is_empty() {
                    str.push_str(&format!(" {name}"))
                } else {
                    match value {
                        PropValue::Literal(v) => str.push_str(&format!(" {name}=\"{v}\"")),
                        PropValue::Expr(v) => str.push_str(&format!(" {name}={{{v}}}")),
                    }
                }
            }
        }
        str
    }
}
