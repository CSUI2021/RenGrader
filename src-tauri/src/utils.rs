use regex::Regex;
use std::{fs::read_to_string, path::Path};

pub fn get_class_name(source_path: &Path) -> String {
    let re_space = Regex::new(r"\s{2,}").unwrap();
    let re = Regex::new(r"public class (.+?) ?\{").unwrap();

    let mut content = read_to_string(source_path).unwrap();
    content = content.replace("\n", " ");

    let cleaned_space = re_space.replace_all(content.as_str(), " ");

    let captured = re.captures(cleaned_space.as_ref()).unwrap();
    let class_name = captured.get(1).expect("Cannot find class name").as_str();
    return class_name.trim().to_owned();
}
