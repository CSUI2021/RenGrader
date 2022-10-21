use anyhow::{Context, Result};
use regex::Regex;
use std::{fs::read_to_string, path::Path};

pub fn get_class_name(source_path: &Path) -> Result<String> {
    let re_space = Regex::new(r"\s{2,}").unwrap();
    let re = Regex::new(r"public class (.+?) ?\{").unwrap();

    let mut content = read_to_string(source_path)?;
    content = content.replace("\n", " ");

    let cleaned_space = re_space.replace_all(content.as_str(), " ");

    let captured = re
        .captures(cleaned_space.as_ref())
        .context("Failed to get match")?;
    let class_name = captured.get(1).context("Failed to get match")?.as_str();
    return Ok(class_name.trim().to_owned());
}

#[macro_export]
macro_rules! exec {
    ($cmd:expr,$cwd:expr,$output:ident,$($args: expr),*) => {
        let mut cmd = Command::new($cmd);
        $(
            cmd.arg($args);
        )*
        cmd.current_dir($cwd);

        let $output;
        match cmd.output() {
            Ok(o) => $output = o,
            Err(_) => return Err("An error has occured".into())
        };
    };
}
