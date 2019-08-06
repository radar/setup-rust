use crate::output::*;
use crate::runner::optional_action;
use std::path::Path;

pub fn bundle() {
    let path = Path::new("Brewfile");
    if path.exists() {
        found("Found Brewfile");
        info("Checking Brewfile dependencies are installed...");
        run_check()
    } else {
        fail("Could not find Brewfile")
    }
}

fn run_check() {
    optional_action("brew", &["bundle", "check"], bundle_installed, bundle_not_installed)
}

fn bundle_installed() {
    found("Bundle installed")
}

fn bundle_not_installed() {
    info("Homebrew bundle not installed. You might want to check the output of 'brew bundle check'");
	info("Alternatively, you could run 'brew bundle install' to install the dependencies listed in the Brewfile")
}
