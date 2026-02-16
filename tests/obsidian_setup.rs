mod spec_runner;

use spec_runner::{fail_fast_from_env, spec_suite_with_flavor, Flavor};

#[test]
fn obsidian_specs() {
    spec_suite_with_flavor("tests/obsidian", Flavor::Obsidian, fail_fast_from_env());
}
