mod spec_runner;

use spec_runner::{fail_fast_from_env, spec_suite_with_flavor, Flavor};

#[test]
fn github_specs() {
    spec_suite_with_flavor("tests/github", Flavor::Github, fail_fast_from_env());
}
