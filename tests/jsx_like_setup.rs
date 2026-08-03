mod spec_runner;

use markdown::parser::ParserOptions;
use spec_runner::{fail_fast_from_env, spec_suite_exact};

#[test]
fn jsx_like_specs() {
    spec_suite_exact(
        "tests/jsx_like",
        ParserOptions::default().enabled_jsx_like_component(),
        fail_fast_from_env(),
    );
}
