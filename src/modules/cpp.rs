use std::process::Command;

use crate::config::SegmentConfig;

use ansi_term::Color;

use super::{Context, Module};

/// Creates a module with the current C++ version
///
/// Will display the Java version if any of the following criteria are met:
///     - Current directory contains a file with a `.c`, `.cpp`, `.h` or `.hpp` extension
///     - Current directory contains a `Makefile` or `CMakeLists.txt` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_cpp_project = context
        .try_begin_scan()?
        .set_files(&["Makefile", "CMakeLists.txt"])
        .set_extensions(&["c", "cpp", "h", "hpp"])
        .is_match();

    if !is_cpp_project {
        return None;
    }

    match get_cpp_version() {
        Some(version) => {
            let formatted_version = format!("v{}", version.trim());
            let mut module = context.new_module("cpp");
            module.set_style(Color::Red.bold());
            module.create_segment("symbol", &SegmentConfig::new("C++ "));
            module.create_segment("version", &SegmentConfig::new(&formatted_version));
            Some(module)
        }
        None => None
    }
}

fn get_cpp_version() -> Option<String> {
    let cpp_command = match std::env::var("CXX") {
        Ok(val) => val,
        Err(_) => String::from("c++")
    };
    match Command::new(cpp_command).arg("-dumpversion").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None
    }
}
