mod spec_runner;

use spec_runner::{Flavor, fail_fast_from_env, spec_suite_with_flavor};

#[test]
fn obsidian_specs() {
    spec_suite_with_flavor("tests/obsidian", Flavor::Obsidian, fail_fast_from_env());
}
