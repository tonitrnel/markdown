mod spec_runner;

use spec_runner::{Flavor, fail_fast_from_env, spec_suite_with_flavor};

#[test]
fn commonmark_specs() {
    spec_suite_with_flavor("tests/commonmark", Flavor::CommonMark, fail_fast_from_env());
}
