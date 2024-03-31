use std::collections::HashMap;

pub type ElementProps = HashMap<String, String>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Html {
    Block(BlockType),
    Inline(InlineType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockType {
    /// **Start condition:** line begins with the string`<pre`,`<script`,`<style`, or`<textarea`(case-insensitive), followed by a space, a tab, the string`>`, or the end of the line.  
    ///
    /// **End condition:** line contains an end tag`</pre>`,`</script>`,`</style>`, or`</textarea>`(case-insensitive; it need not match the start tag).
    Type1 = 1,
    /// **Start condition:** line begins with the string `<!--`.
    ///
    /// **End condition:** line contains the string `-->`.
    Type2 = 2,
    /// **Start condition:** line begins with the string `<?`.
    ///
    /// **End condition:** line contains the string `?>`.
    Type3 = 3,
    /// **Start condition:** line begins with the string `<!` followed by an ASCII letter.
    ///
    /// **End condition:** line contains the character `>`.
    Type4 = 4,
    /// **Start condition:** line begins with the string `<![CDATA[`.
    ///
    /// **End condition:** line contains the string `]]>`.
    Type5 = 5,
    /// **Start condition:** line begins with the string`<`or`</`followed by one of the strings (case-insensitive)`address`,`article`,`aside`,`base`,`basefont`,`blockquote`,`body`,`caption`,`center`,`col`,`colgroup`,`dd`,`details`,`dialog`,`dir`,`div`,`dl`,`dt`,`fieldset`,`figcaption`,`figure`,`footer`,`form`,`frame`,`frameset`,`h1`,`h2`,`h3`,`h4`,`h5`,`h6`,`head`,`header`,`hr`,`html`,`iframe`,`legend`,`li`,`link`,`main`,`menu`,`menuitem`,`nav`,`noframes`,`ol`,`optgroup`,`option`,`p`,`param`,`search`,`section`,`summary`,`table`,`tbody`,`td`,`tfoot`,`th`,`thead`,`title`,`tr`,`track`,`ul`, followed by a space, a tab, the end of the line, the string`>`, or the string`/>`.
    ///
    /// **End condition:** line is followed by a blank line.
    Type6 = 6,
    /// **Start condition:** line begins with a complete open tag (with any tag name other than `pre`, `script`, `style`, or `textarea`) or a complete closing tag, followed by zero or more spaces and tabs, followed by the end of the line.
    ///
    /// **End condition:** line is followed by a blank line.
    Type7 = 7,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineType {
    /// type 6
    Standard,
    /// type 7
    Raw,
    /// External, in support of jsx
    External,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    pub tag: String,
    pub props: Option<ElementProps>,
    pub children: Vec<String>,
}
