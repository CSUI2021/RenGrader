#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod utils;

use std::{
    fs::{self, copy, create_dir_all, read_to_string},
    path::{Path, PathBuf},
    process::Command,
    str,
};

use minijinja::render;
use tauri::Window;
use utils::get_class_name;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[derive(Clone, serde::Serialize)]
struct TestResult {
    error: bool,
    message: String,
}

#[tauri::command]
fn run_tests(
    window: Window,
    handle: tauri::AppHandle,
    source_path: &str,
    test_cases_path: &str,
    timeout: i32,
) -> TestResult {
    let mut tmp_dir = handle.path_resolver().app_dir().unwrap();
    tmp_dir.push("tmp");
    create_dir_all(tmp_dir.as_path()).unwrap();

    window
        .emit(
            "onDebugMessage",
            Payload {
                message: format!("Copying {} to {}", source_path, tmp_dir.to_str().unwrap()),
            },
        )
        .unwrap();

    let file_name = Path::new(source_path).file_name().unwrap();
    let target_temp_path: PathBuf = [&tmp_dir, &PathBuf::from(&file_name)].iter().collect();
    copy(source_path, target_temp_path.as_path()).unwrap();

    window
        .emit(
            "onDebugMessage",
            Payload {
                message: "Getting java class name".into(),
            },
        )
        .unwrap();

    let class_name = get_class_name(target_temp_path.as_path());

    window
        .emit(
            "onDebugMessage",
            Payload {
                message: format!("Java class name: {}", class_name),
            },
        )
        .unwrap();

    let resource_path = handle
        .path_resolver()
        .resolve_resource("java/RenGrader.java")
        .expect("failed to resolve resource");
    let base_program = read_to_string(resource_path).unwrap();

    let escaped_path = test_cases_path.to_string().replace(r"\", r"\\");
    let result = render!(
        base_program.as_str(),
        CLASS_NAME => class_name,
        TEST_CASE_DIR => escaped_path,
        TIMEOUT => timeout.to_string()
    );

    let target_temp_path: PathBuf = [&tmp_dir, &PathBuf::from("RenGrader.java")]
        .iter()
        .collect();
    fs::write(target_temp_path.as_path(), result).unwrap();

    window
        .emit(
            "onDebugMessage",
            Payload {
                message: "Compiling java source code".into(),
            },
        )
        .unwrap();

    let mut javac = Command::new("javac");
    javac.arg(file_name);
    javac.arg("RenGrader.java");
    javac.current_dir(tmp_dir.as_path());

    let output = javac.output().expect("Something fucky happened");
    if !output.status.success() {
        return TestResult {
            error: true,
            message: str::from_utf8(&output.stderr).unwrap().into(),
        };
    }

    window
        .emit(
            "onDebugMessage",
            Payload {
                message: format!("Java output: {}", str::from_utf8(&output.stdout).unwrap()),
            },
        )
        .unwrap();

    window
        .emit(
            "onDebugMessage",
            Payload {
                message: "Running actual grader".into(),
            },
        )
        .unwrap();

    let mut java = Command::new("java");
    java.arg("RenGrader");
    java.current_dir(tmp_dir.as_path());

    let output = java.output().expect("Something fucky happened");
    if !output.status.success() {
        return TestResult {
            error: true,
            message: str::from_utf8(&output.stderr).unwrap().into(),
        };
    }

    return TestResult {
        error: false,
        message: str::from_utf8(&output.stdout).unwrap().into(),
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_tests])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
