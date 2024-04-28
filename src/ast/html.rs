use std::borrow::Cow;
use std::collections::HashMap;

pub type ElementProps = HashMap<String, String>;

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    pub name: String,
    pub props: Option<ElementProps>,
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
            props: if let Some(props) = props.filter(|it| !it.is_empty()) {
                let mut map = HashMap::new();
                for prop in props {
                    map.insert(prop.0, prop.1.to_string());
                }
                Some(map)
            } else {
                None
            },
        }
    }
    pub(crate) fn attr_str(&self) -> String {
        let mut str = String::new();
        if let Some(props) = &self.props {
            for (name, value) in props.iter() {
                str.push_str(&format!(" {name}=\"{value}\""))
            }
        }
        str
    }
}
