mod spec_runner;

use markdown::ParserOptions;
use spec_runner::{fail_fast_from_env, spec_suite};

#[test]
fn cjk_friendly_specs() {
    spec_suite(
        "tests/cjk_friendly",
        ParserOptions::default()
            .enabled_gfm()
            .enabled_cjk_friendly_delimiters(),
        fail_fast_from_env(),
    );
}
