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
            HtmlType::Type1(e, _) | HtmlType::Type6(e, _) | HtmlType::Type7(e, _) => {
                DISALLOWED_TAG_NAMES
                    .iter()
                    .any(|it| e.name.eq_ignore_ascii_case(it))
            }
            _ => false,
        }
    }
    pub(crate) fn set_flag_is_full(&mut self) {
        let _type = match self {
            Html::Block(t) | Html::Inline(t) => t,
        };
        match _type {
            HtmlType::Type1(_, flag) | HtmlType::Type6(_, flag) | HtmlType::Type7(_, flag) => {
                *flag = Flag::Full
            }
            _ => (),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlType {
    /// HTML Elements(pre, script, style, textarea)
    ///
    /// - Start condition: line begins with the string`<pre`,`<script`,`<style`, or`<textarea`(case-insensitive), followed by a space, a tab, the string`>`, or the end of the line.  
    /// - End condition: line contains an end tag`</pre>`,`</script>`,`</style>`, or`</textarea>`(case-insensitive; it need not match the start tag).
    Type1(Element, Flag),

    ///  HTML Comments
    ///
    /// - Start condition: line begins with the string `<!--`.
    /// - End condition  : line contains the string `-->`.
    Type2,

    /// Processing Instruction
    ///
    /// - Start condition: line begins with the string `<?`.
    /// - End condition  : line contains the string `?>`.
    Type3,

    /// Declaration
    ///
    /// - Start condition: line begins with the string `<!` followed by an ASCII letter.
    /// - End condition  : line contains the character `>`.
    Type4,

    /// CDATA
    ///
    /// - Start condition: line begins with the string `<![CDATA[`.
    /// - End condition  : line contains the string `]]>`.
    Type5,

    /// HTML canonical elements
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
    Type6(Element, Flag),

    /// Other Non-HTML canonical elements
    ///
    /// - Start condition: line begins with a complete open tag (with any tag name other than `pre`, `script`, `style`, or `textarea`) or a complete closing tag, followed by zero or more spaces and tabs, followed by the end of the line.
    /// - End condition:
    ///    1. line is followed by a blank line.
    ///    2. find the closing tag that matches(custom rule, non cfm spec).
    Type7(Element, Flag),
}

impl Serialize for HtmlType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut obj = serializer.serialize_map(Some(4))?;
        match self {
            HtmlType::Type1(..) => obj.serialize_entry("type", "type1")?,
            HtmlType::Type2 => obj.serialize_entry("type", "type2")?,
            HtmlType::Type3 => obj.serialize_entry("type", "type3")?,
            HtmlType::Type4 => obj.serialize_entry("type", "type4")?,
            HtmlType::Type5 => obj.serialize_entry("type", "type5")?,
            HtmlType::Type6(..) => obj.serialize_entry("type", "type6")?,
            HtmlType::Type7(..) => obj.serialize_entry("type", "type7")?,
        }
        match self {
            HtmlType::Type1(element, flag)
            | HtmlType::Type6(element, flag)
            | HtmlType::Type7(element, flag) => {
                obj.serialize_entry("name", &element.name)?;
                obj.serialize_entry("props", &element.props)?;
                obj.serialize_entry("flag", flag)?;
            }
            _ => (),
        }
        obj.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    pub name: String,
    pub props: Option<Vec<(String, String)>>,
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
            props: props
                .filter(|it| !it.is_empty())
                .map(|props| props.into_iter().map(|(n, v)| (n, v.to_string())).collect()),
        }
    }
    #[cfg(feature = "html")]
    pub(crate) fn attr_str(&self) -> String {
        let mut str = String::new();
        if let Some(props) = &self.props {
            for (name, value) in props.iter() {
                if value.is_empty() {
                    str.push_str(&format!(" {name}"))
                } else {
                    str.push_str(&format!(" {name}=\"{value}\""))
                }
            }
        }
        str
    }
}
