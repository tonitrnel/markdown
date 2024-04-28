use crate::ast::MarkdownNode;
use crate::inlines::ProcessCtx;
use crate::tokenizer::Token;

pub(super) fn process(
    ProcessCtx {
        id, line, parser, ..
    }: &mut ProcessCtx,
) -> bool {
    let start_location = line.start_location();
    let mut end = 0;
    line.next();
    for (i, item) in line.iter().enumerate() {
        match item.token {
            Token::Text(..) | Token::Digit(..) => continue,
            Token::Underscore | Token::Plus | Token::Hyphen => continue,
            Token::Colon if i > 0 => {
                end = i;
                break;
            }
            _ => return false,
        }
    }
    if end == 0 {
        return false;
    }
    let end_location = line.end_location();
    parser.append_to(
        *id,
        MarkdownNode::Emoji(line.slice(0, end).to_string()),
        (start_location, end_location),
    );
    line.skip(end + 1);
    true
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn it_works() {
        let ast =
            Parser::new("@octocat :+1: This PR looks great - it's ready to merge! :狗头:").parse();
        println!("{ast:?}")
    }
}
