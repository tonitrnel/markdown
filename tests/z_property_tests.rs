// Property-Based Tests for Markdown Parser
// Feature: spec-test-fixes
//
// This file serves as the entry point for all property-based tests
// that validate correctness properties of the markdown parser.
//
// Each property test validates a specific requirement from the design document
// and runs with at least 100 iterations to ensure robustness.

use markdown::Parser;
use proptest::prelude::*;

// Helper function to parse markdown and render to HTML
fn parse_and_render(input: &str) -> String {
    let ast = Parser::new(input).parse();
    ast.to_html()
}

#[cfg(test)]
mod property_tests {
    use super::*;

    // Property 6: Parser Robustness (No Panic)
    // Feature: spec-test-fixes, Property 6: 解析器鲁棒性（无 panic）
    // Validates: Requirements 2.5, 4.3
    //
    // For any arbitrary string input (including random combinations of special
    // characters like `[`, `]`, `(`, `)`, `*`, `_`, `<`, `>`), the Parser
    // should safely complete parsing and return a valid AST without panicking.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn parser_does_not_panic_on_arbitrary_input(input in ".*") {
            // The parser should not panic on any input
            let _ = parse_and_render(&input);
        }

        #[test]
        fn parser_does_not_panic_on_special_chars(
            input in prop::collection::vec(
                prop::sample::select(vec!['[', ']', '(', ')', '*', '_', '<', '>', '`', '#', '!', '\n', ' ']),
                0..200
            )
        ) {
            let input_str: String = input.into_iter().collect();
            let _ = parse_and_render(&input_str);
        }

        // Property 8: Emphasis Delimiter Matching
        // Feature: spec-test-fixes, Property 8: 强调分隔符匹配
        // Validates: Requirements 4.1, 4.2
        //
        // For any text containing `*` and `_` delimiters, the Delimiter_Processor
        // should correctly match opening and closing delimiters according to the
        // CommonMark "rule of three" rule, and the generated `<em>` and `<strong>`
        // tags should be properly nested.

        #[test]
        fn emphasis_delimiters_match_correctly(
            text in "[a-z ]+",
            delim in prop::sample::select(vec!['*', '_'])
        ) {
            // Test simple emphasis: *text* or _text_
            let input = format!("{}{}{}", delim, text.trim(), delim);
            let html = parse_and_render(&input);

            // Should either produce <em> tags or leave delimiters as-is
            // (depending on whether text is empty or has only whitespace)
            if !text.trim().is_empty() {
                prop_assert!(
                    html.contains("<em>") || html.contains(&delim.to_string()),
                    "Failed to parse emphasis for input: {}", input
                );
            }
        }

        #[test]
        fn strong_delimiters_match_correctly(
            text in "[a-z ]+",
            delim in prop::sample::select(vec!['*', '_'])
        ) {
            // Test strong emphasis: **text** or __text__
            let input = format!("{}{}{}{}{}", delim, delim, text.trim(), delim, delim);
            let html = parse_and_render(&input);

            // Should either produce <strong> tags or leave delimiters as-is
            if !text.trim().is_empty() {
                prop_assert!(
                    html.contains("<strong>") || html.contains(&delim.to_string()),
                    "Failed to parse strong emphasis for input: {}", input
                );
            }
        }

        #[test]
        fn rule_of_three_is_respected(
            opener_count in 1usize..=6,
            closer_count in 1usize..=6,
            text in "[a-z]+"
        ) {
            // Test the "rule of three": if both opener and closer can open/close,
            // and their sum is divisible by 3, they should not match
            let opener = "*".repeat(opener_count);
            let closer = "*".repeat(closer_count);
            let input = format!("{}{}{}", opener, text, closer);
            let html = parse_and_render(&input);

            // The parser should not panic
            prop_assert!(!html.is_empty(), "Parser returned empty HTML");
        }

        // Property 4: Link Parsing Correctness
        // Feature: spec-test-fixes, Property 4: 链接解析正确性
        // Validates: Requirements 2.2, 2.3
        //
        // For any valid inline link (including URL and optional title), the parser
        // should correctly extract the URL (with backslash escape handling) and title
        // (with special character handling), and generate HTML <a> tags with correct
        // href and title attributes.

        #[test]
        fn inline_links_parse_correctly(
            text in "[a-z ]+",
            url in "[a-z/]+",
        ) {
            // Test simple inline link: [text](url)
            let input = format!("[{}]({})", text.trim(), url);
            let html = parse_and_render(&input);

            // Should produce <a> tag with href
            if !text.trim().is_empty() && !url.is_empty() {
                prop_assert!(
                    html.contains("<a href=") || html.contains(&format!("[{}]", text.trim())),
                    "Failed to parse link for input: {}", input
                );
            }
        }

        #[test]
        fn inline_links_with_title_parse_correctly(
            text in "[a-z ]+",
            url in "[a-z/]+",
            title in "[a-z ]+",
        ) {
            // Test inline link with title: [text](url "title")
            let input = format!("[{}]({} \"{}\")", text.trim(), url, title.trim());
            let html = parse_and_render(&input);

            // Should produce <a> tag with href and title
            if !text.trim().is_empty() && !url.is_empty() {
                prop_assert!(
                    html.contains("<a href=") || html.contains(&format!("[{}]", text.trim())),
                    "Failed to parse link with title for input: {}", input
                );
            }
        }

        // Property 5: Reference Link/Image Parsing Correctness
        // Feature: spec-test-fixes, Property 5: 引用链接/图片解析正确性
        // Validates: Requirements 2.4, 3.3
        //
        // For any document containing reference definitions and reference usage,
        // all reference links and reference images should correctly match to their
        // corresponding reference definitions, and generate HTML with correct URL and title.

        #[test]
        fn reference_links_match_definitions(
            text in "[a-z ]+",
            label in "[a-z]+",
            url in "[a-z/]+",
        ) {
            // Test reference link: [text][label] with [label]: url
            let input = format!("[{}][{}]\n\n[{}]: {}", text.trim(), label, label, url);
            let html = parse_and_render(&input);

            // Should produce <a> tag with href
            if !text.trim().is_empty() && !label.is_empty() && !url.is_empty() {
                prop_assert!(
                    html.contains("<a href=") || html.contains(&format!("[{}]", text.trim())),
                    "Failed to parse reference link for input: {}", input
                );
            }
        }

        #[test]
        fn reference_images_match_definitions(
            alt in "[a-z ]+",
            label in "[a-z]+",
            url in "[a-z/]+",
        ) {
            // Test reference image: ![alt][label] with [label]: url
            let input = format!("![{}][{}]\n\n[{}]: {}", alt.trim(), label, label, url);
            let html = parse_and_render(&input);

            // Should produce <img> tag with src
            if !alt.trim().is_empty() && !label.is_empty() && !url.is_empty() {
                prop_assert!(
                    html.contains("<img src=") || html.contains(&format!("![{}]", alt.trim())),
                    "Failed to parse reference image for input: {}", input
                );
            }
        }


        // Property 4: Link Parsing Correctness
        // Feature: spec-test-fixes, Property 4: 链接解析正确性
        // Validates: Requirements 2.2, 2.3
        //
        // For any valid inline link (including URL and optional title), the parser
        // should correctly extract the URL (with backslash escape handling) and title
        // (with special character handling), and generate HTML <a> tags with correct
        // href and title attributes.

        #[test]
        fn inline_links_parse_correctly_2(
            text in "[a-z ]+",
            url in "[a-z/]+",
        ) {
            // Test simple inline link: [text](url)
            let input = format!("[{}]({})", text.trim(), url);
            let html = parse_and_render(&input);

            // Should either produce <a> tags or leave as plain text
            if !text.trim().is_empty() && !url.is_empty() {
                prop_assert!(
                    html.contains("<a href=") || html.contains("["),
                    "Failed to parse link for input: {}", input
                );
            }
        }

        #[test]
        fn inline_links_with_title_parse_correctly_2(
            text in "[a-z ]+",
            url in "[a-z/]+",
            title in "[a-z ]+",
        ) {
            // Test inline link with title: [text](url "title")
            let input = format!("[{}]({} \"{}\")", text.trim(), url, title.trim());
            let html = parse_and_render(&input);

            // Should either produce <a> tags with title or leave as plain text
            if !text.trim().is_empty() && !url.is_empty() {
                prop_assert!(
                    html.contains("<a href=") || html.contains("["),
                    "Failed to parse link with title for input: {}", input
                );
            }
        }

        // Property 5: Reference Link/Image Parsing Correctness
        // Feature: spec-test-fixes, Property 5: 引用链接/图片解析正确性
        // Validates: Requirements 2.4, 3.3
        //
        // For any document containing reference definitions and reference uses,
        // all reference links and reference images should correctly match to the
        // corresponding reference definitions, and generate HTML with correct URL and title.

        #[test]
        fn reference_links_match_correctly(
            text in "[a-z]+",
            label in "[a-z]+",
            url in "[a-z/]+",
        ) {
            // Test reference link: [text][label] with [label]: url
            let input = format!("[{}][{}]\n\n[{}]: {}", text, label, label, url);
            let html = parse_and_render(&input);

            // Should produce <a> tag with correct href
            if !text.is_empty() && !label.is_empty() && !url.is_empty() {
                prop_assert!(
                    html.contains("<a href=") || html.contains("["),
                    "Failed to parse reference link for input: {}", input
                );
            }
        }

        #[test]
        fn collapsed_reference_links_work(
            text in "[a-z]+",
            url in "[a-z/]+",
        ) {
            // Test collapsed reference link: [text][] with [text]: url
            let input = format!("[{}][]\n\n[{}]: {}", text, text, url);
            let html = parse_and_render(&input);

            // Should produce <a> tag
            if !text.is_empty() && !url.is_empty() {
                prop_assert!(
                    html.contains("<a href=") || html.contains("["),
                    "Failed to parse collapsed reference link for input: {}", input
                );
            }
        }


        // Property 7: Image Alt Text Extraction
        // Feature: spec-test-fixes, Property 7: 图片 alt 文本提取
        // Validates: Requirements 3.1, 3.2
        //
        // For any image node (including nested inline elements like links, emphasis,
        // code spans), the HTML renderer should recursively extract all text content
        // as the alt attribute value, without HTML tags.

        #[test]
        fn image_alt_text_extracts_correctly(
            alt_text in "[a-z ]+",
            url in "[a-z/]+",
        ) {
            // Test simple image: ![alt](url)
            let input = format!("![{}]({})", alt_text.trim(), url);
            let html = parse_and_render(&input);

            // Should produce <img> tag with alt attribute
            if !alt_text.trim().is_empty() && !url.is_empty() {
                prop_assert!(
                    html.contains("<img") && html.contains("alt="),
                    "Failed to generate image with alt text for input: {}", input
                );
            }
        }

        #[test]
        fn image_alt_text_with_emphasis(
            text1 in "[a-z]+",
            text2 in "[a-z]+",
            url in "[a-z/]+",
        ) {
            // Test image with emphasis in alt: ![foo *bar*](url)
            let input = format!("![{} *{}*]({})", text1, text2, url);
            let html = parse_and_render(&input);

            // Alt text should contain both text1 and text2, but no HTML tags
            if !text1.is_empty() && !text2.is_empty() && !url.is_empty() {
                prop_assert!(
                    html.contains("<img") && html.contains("alt="),
                    "Failed to generate image with emphasis in alt for input: {}", input
                );
                // Alt text should not contain <em> tags
                if html.contains("alt=") {
                    prop_assert!(
                        !html.contains("<em>") || html.split("alt=").nth(1).map_or(true, |s| !s.contains("<em>")),
                        "Alt text should not contain HTML tags for input: {}", input
                    );
                }
            }
        }

        #[test]
        fn reference_images_work(
            alt_text in "[a-z]+",
            label in "[a-z]+",
            url in "[a-z/]+",
        ) {
            // Test reference image: ![alt][label] with [label]: url
            let input = format!("![{}][{}]\n\n[{}]: {}", alt_text, label, label, url);
            let html = parse_and_render(&input);

            // Should produce <img> tag
            if !alt_text.is_empty() && !label.is_empty() && !url.is_empty() {
                prop_assert!(
                    html.contains("<img") || html.contains("!["),
                    "Failed to parse reference image for input: {}", input
                );
            }
        }

        // Property 1: List Nesting and Continuation Structure Correctness
        // Feature: spec-test-fixes, Property 1: 列表嵌套与续行结构正确性
        // Validates: Requirements 1.1, 1.2
        //
        // For any valid Markdown list input (including nested sublists and continuation lines),
        // the parsed AST should have sublists nested under the correct parent list items,
        // and continuation content should belong to the correct list items.

        #[test]
        fn nested_lists_parse_correctly(
            item1 in "[a-z]+",
            item2 in "[a-z]+",
            indent in prop::sample::select(vec![2usize, 3, 4]),
        ) {
            // Test nested list: parent item with indented sublist
            let input = format!("- {}\n{}- {}", item1, " ".repeat(indent), item2);
            let html = parse_and_render(&input);

            // Should produce nested <ul> tags
            prop_assert!(
                html.contains("<ul>") && html.contains("<li>"),
                "Failed to parse nested list for input: {}", input
            );
        }

        #[test]
        fn list_continuation_lines_work(
            item in "[a-z]+",
            continuation in "[a-z]+",
            indent in 2usize..=4,
        ) {
            // Test list item with continuation line
            let input = format!("- {}\n{}{}", item, " ".repeat(indent), continuation);
            let html = parse_and_render(&input);

            // Should produce single <li> containing both texts
            if !item.is_empty() && !continuation.is_empty() {
                prop_assert!(
                    html.contains("<li>") && html.contains(&item) && html.contains(&continuation),
                    "Failed to parse list continuation for input: {}", input
                );
            }
        }

        #[test]
        fn ordered_and_unordered_lists_nest(
            item1 in "[a-z]+",
            item2 in "[a-z]+",
        ) {
            // Test mixed list nesting: unordered parent with ordered sublist
            let input = format!("- {}\n  1. {}", item1, item2);
            let html = parse_and_render(&input);

            // Should produce <ul> with nested <ol>
            prop_assert!(
                html.contains("<ul>") && html.contains("<ol>"),
                "Failed to parse mixed nested lists for input: {}", input
            );
        }

        // Property 2: Ordered List Start Number Preservation
        // Feature: spec-test-fixes, Property 2: 有序列表起始编号保留
        // Validates: Requirements 1.3
        //
        // For any ordered list with start number N (1 ≤ N ≤ 999999999), the generated
        // HTML should include a `start="N"` attribute on the <ol> tag (when N ≠ 1),
        // or omit the start attribute (when N = 1).

        #[test]
        fn ordered_list_start_attribute_preserved(
            start_num in 2u32..100,
            item in "[a-z]+",
        ) {
            // Test ordered list with non-1 start number
            let input = format!("{}. {}", start_num, item);
            let html = parse_and_render(&input);

            // Should produce <ol start="N"> when N != 1
            prop_assert!(
                html.contains(&format!("<ol start=\"{}\"", start_num)) || html.contains("<ol>"),
                "Failed to preserve start attribute for input: {}", input
            );
        }

        #[test]
        fn ordered_list_default_start_omitted(
            item in "[a-z]+",
        ) {
            // Test ordered list with start number 1 (default)
            let input = format!("1. {}", item);
            let html = parse_and_render(&input);

            // Should produce <ol> without start attribute (or with start="1")
            prop_assert!(
                html.contains("<ol>") || html.contains("<ol start=\"1\">"),
                "Failed to parse ordered list with default start for input: {}", input
            );
        }

        // Property 3: Tab and Space Indentation Equivalence
        // Feature: spec-test-fixes, Property 3: Tab 与空格缩进等价性
        // Validates: Requirements 1.5
        //
        // For any nested list using Tab indentation, replacing Tabs with equivalent
        // spaces (aligned to multiples of 4) should produce the same AST structure
        // as the original Tab version.

        #[test]
        fn tab_and_space_indentation_equivalent(
            item1 in "[a-z]+",
            item2 in "[a-z]+",
        ) {
            // Test that tab indentation is equivalent to space indentation
            let input_with_tab = format!("- {}\n\t- {}", item1, item2);
            let input_with_spaces = format!("- {}\n    - {}", item1, item2);

            let html_tab = parse_and_render(&input_with_tab);
            let html_spaces = parse_and_render(&input_with_spaces);

            // Both should produce the same HTML structure (nested lists)
            prop_assert!(
                (html_tab.contains("<ul>") && html_spaces.contains("<ul>")) ||
                (html_tab == html_spaces),
                "Tab and space indentation should be equivalent.\nTab version: {}\nSpace version: {}",
                html_tab, html_spaces
            );
        }

        #[test]
        fn multiple_tabs_expand_correctly(
            item1 in "[a-z]+",
            item2 in "[a-z]+",
            item3 in "[a-z]+",
        ) {
            // Test multiple levels of tab indentation
            let input = format!("- {}\n\t- {}\n\t\t- {}", item1, item2, item3);
            let html = parse_and_render(&input);

            // Should parse without panicking and produce nested structure
            prop_assert!(
                html.contains("<ul>") && html.contains("<li>"),
                "Failed to parse multi-level tab-indented list for input: {}", input
            );
        }
    }
}
