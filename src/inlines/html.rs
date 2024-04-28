use crate::ast::{html, MarkdownNode};
use crate::blocks::html::scan_html_type;
use crate::inlines::ProcessCtx;

pub(super) fn process(
    ProcessCtx {
        id, line, parser, ..
    }: &mut ProcessCtx,
) -> bool {
    let start_location = line.start_location();
    // println!(
    //     "inlines::html::process {:?} {:?}",
    //     line,
    //     line.get_raw(line.start_offset + 1)
    // );
    let (_, len, html_type) = if let Some(html_type) = scan_html_type(line, true) {
        html_type
    } else {
        return false;
    };
    // println!("inlines::html::process len = {len} html_type = {html_type:?}")
    let end_location = line[len - 1].end_location();
    line.skip(len);
    let (start, end, html) = match &html_type {
        html::HtmlType::Type2
        | html::HtmlType::Type3
        | html::HtmlType::Type4
        | html::HtmlType::Type5
        | html::HtmlType::Type1(.., html::Flag::SelfClose | html::Flag::End)
        | html::HtmlType::Type6(.., html::Flag::SelfClose | html::Flag::End)
        | html::HtmlType::Type7(.., html::Flag::SelfClose | html::Flag::End) => {
            (0, 0, html::Html::Inline(html_type))
        }
        html::HtmlType::Type1(..) | html::HtmlType::Type6(..) | html::HtmlType::Type7(..) => {
            let mut html = html::Html::Inline(html_type);
            let (start, end) = if let Some(r) = html.scan_end(line) {
                r
            } else {
                return false;
            };
            html.set_flag_is_full();
            (start, end, html)
        }
    };
    // println!("inlines::html::process html = {html:?}")
    let idx = parser.append_to(
        *id,
        MarkdownNode::Html(html),
        (start_location, end_location),
    );
    if start > 0 {
        let text_line = line.slice(0, start);
        parser.append_text_to(
            idx,
            text_line.to_string(),
            (text_line.start_location(), text_line.end_location()),
        );
    }
    if end > 0 {
        line.skip(end);
    };
    true
}
