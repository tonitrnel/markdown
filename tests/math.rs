use markdown::ParserOptions;
use markdown::parser::Parser;

#[test]
fn test_inline_math() {
    let input = "This is inline math: $E = mc^2$";
    let options = ParserOptions::default().enabled_gfm();
    let ast = Parser::new_with_options(input, options).parse();
    let output = ast.to_html();
    assert!(output.contains("<span class=\"math math-inline\">"));
    assert!(output.contains("E = mc^2"));
}

#[test]
fn test_display_math() {
    let input = "$$E = mc^2$$";
    let options = ParserOptions::default().enabled_gfm();
    let ast = Parser::new_with_options(input, options).parse();
    let output = ast.to_html();
    assert!(output.contains("<div class=\"math math-display\">"));
    assert!(output.contains("E = mc^2"));
}

#[test]
fn test_matrix_math() {
    let input = r#"$$\begin{vmatrix}a & b\\c & d\end{vmatrix}=ad-bc$$"#;
    let options = ParserOptions::default().enabled_gfm();
    let ast = Parser::new_with_options(input, options).parse();
    let output = ast.to_html();
    assert!(output.contains("<div class=\"math math-display\">"));
    assert!(output.contains(r"\begin{vmatrix}"));
    assert!(output.contains(r"\end{vmatrix}"));
    assert!(output.contains("ad-bc"));
}

#[test]
fn test_multiline_display_math() {
    let input = "$$\\begin{align}\nx &= a + b \\\\\ny &= c + d\n\\end{align}$$";
    let options = ParserOptions::default().enabled_gfm();
    let ast = Parser::new_with_options(input, options).parse();
    let output = ast.to_html();
    assert!(output.contains("<div class=\"math math-display\">"));
    assert!(output.contains(r"\begin{align}"));
    assert!(output.contains("y &amp;= c + d"));
}

#[test]
fn test_multiline_display_math_user_case() {
    let input = "$$\\begin{vmatrix}a & b\\\\\nc & d\n\\end{vmatrix}=ad-bc$$";
    let options = ParserOptions::default().enabled_gfm();
    let ast = Parser::new_with_options(input, options).parse();
    let output = ast.to_html();
    assert!(output.starts_with("<div class=\"math math-display\">"));
    assert!(output.contains(r"\begin{vmatrix}"));
    assert!(output.contains("c &amp; d"));
    assert!(output.contains(r"\end{vmatrix}=ad-bc"));
    assert!(!output.contains("<p><div class=\"math math-display\">"));
}

#[test]
fn test_math_with_text() {
    let input = "The equation $x^2 + y^2 = r^2$ represents a circle.";
    let options = ParserOptions::default().enabled_gfm();
    let ast = Parser::new_with_options(input, options).parse();
    let output = ast.to_html();
    assert!(output.contains("The equation"));
    assert!(output.contains("<span class=\"math math-inline\">"));
    assert!(output.contains("x^2 + y^2 = r^2"));
    assert!(output.contains("represents a circle"));
}

#[test]
fn test_escaped_dollar() {
    let input = r"This is not math: \$100";
    let options = ParserOptions::default().enabled_gfm();
    let ast = Parser::new_with_options(input, options).parse();
    let output = ast.to_html();
    assert!(!output.contains("<span class=\"math"));
    assert!(output.contains("$100"));
}

#[test]
fn test_multiple_inline_math() {
    let input = "First: $a + b$, second: $c + d$";
    let options = ParserOptions::default().enabled_gfm();
    let ast = Parser::new_with_options(input, options).parse();
    let output = ast.to_html();
    let math_count = output.matches("<span class=\"math math-inline\">").count();
    assert_eq!(math_count, 2);
}
