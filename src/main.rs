mod homebrew;
mod toolversions;

use output::{found, info};

mod output;
mod runner;
mod installers;

fn main() {
    // homebrew::bundle();

    if toolversions::present() {
        found("Found a .tool-versions file, will check those packages are installed...");
        let versions = toolversions::parse();
    } else {
        info("Could not find a .tool-versions file");
        info("Setup uses .tool-versions to determine what languages to install.");
        info("Please follow the ASDF instructions for creating a .tool-versions file.");
        info("https://asdf-vm.com/#/core-configuration?id=tool-versions");
    }
}
