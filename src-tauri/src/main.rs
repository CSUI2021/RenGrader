#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod utils;

use std::{
    fs::{self, copy, create_dir_all, read_to_string, ReadDir},
    path::{Path, PathBuf},
    process::Command,
    str,
};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

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

#[tauri::command(async)]
fn run_tests(
    window: Window,
    handle: tauri::AppHandle,
    source_path: &str,
    test_cases_path: &str,
    timeout: i32,
) -> Result<TestResult, String> {
    macro_rules! log_frontend {
        ($a:expr) => {{
            window
                .emit("onDebugMessage", Payload { message: $a })
                .unwrap_or(());
        }};
    }

    let mut tmp_dir;
    match handle.path_resolver().app_dir() {
        Some(dir) => tmp_dir = dir,
        None => return Err("Cannot get app data directory".into()),
    }

    tmp_dir.push("tmp");
    match create_dir_all(tmp_dir.as_path()) {
        Ok(_) => (),
        Err(err) => {
            log_frontend!(err.to_string());
            return Err("Cannot create temporary directory".into());
        }
    };

    log_frontend!(format!(
        "Copying {} to {}",
        source_path,
        tmp_dir.to_str().unwrap()
    ));

    let file_name;
    match Path::new(source_path).file_name() {
        Some(name) => file_name = name,
        None => return Err("Cannot get file name".into()),
    }

    let target_temp_path: PathBuf = [&tmp_dir, &PathBuf::from(&file_name)].iter().collect();
    match copy(source_path, target_temp_path.as_path()) {
        Ok(_) => (),
        Err(err) => {
            log_frontend!(err.to_string());
            return Err("Failed to copy java source code to temporary directory".into());
        }
    }

    log_frontend!("Getting java class name".into());
    let class_name;
    match get_class_name(target_temp_path.as_path()) {
        Ok(cname) => class_name = cname,
        Err(err) => {
            log_frontend!(err.to_string());
            return Err("Failed to get class name, is your java file correct?".into());
        }
    };

    log_frontend!(format!("Java class name: {}", class_name));
    let resource_path = handle
        .path_resolver()
        .resolve_resource("java/RenGrader.java")
        .expect("Failed to resolve resource");
    let base_program = read_to_string(resource_path).unwrap();

    let input_dir: PathBuf = [&test_cases_path, "in"].iter().collect();
    let paths: ReadDir;
    match fs::read_dir(input_dir) {
        Ok(p) => paths = p,
        Err(err) => {
            log_frontend!(err.to_string());
            return Err("Failed to iterate through input directories".into());
        }
    }

    let mut grader_result = String::new();
    let escaped_path = test_cases_path.to_string().replace(r"\", r"\\");

    for path in paths {
        let dir_entry = path.unwrap();
        log_frontend!(format!(
            "Running for testcase: {}",
            &dir_entry.file_name().to_str().unwrap()
        )
        .into());

        let full_path_escaped = match dir_entry.path().to_str() {
            None => return Err("Cannot get path".into()),
            Some(s) => s.replace(r"\", r"\\"),
        };
        let result = render!(
            base_program.as_str(),
            INPUT_PATH => full_path_escaped,
            CLASS_NAME => class_name,
            TEST_CASE_DIR => escaped_path,
            TIMEOUT => timeout.to_string()
        );

        let target_temp_path: PathBuf = [&tmp_dir, &PathBuf::from("RenGrader.java")]
            .iter()
            .collect();
        match fs::write(target_temp_path.as_path(), result) {
            Ok(_) => (),
            Err(err) => {
                log_frontend!(err.to_string());
                return Err("Cannot write grader code".into());
            }
        }

        log_frontend!("Compiling java source code".into());
        exec!(
            "javac",
            tmp_dir.as_path(),
            output,
            file_name,
            "RenGrader.java"
        );
        if !output.status.success() {
            return Err(str::from_utf8(&output.stderr)
                .unwrap_or("Cannot decode output".into())
                .into());
        }

        log_frontend!("Running actual grader".into());
        exec!("java", tmp_dir.as_path(), output, "RenGrader");
        if !output.status.success() {
            return Err(str::from_utf8(&output.stderr)
                .unwrap_or("Cannot decode output".into())
                .into());
        }

        let program_result;
        match str::from_utf8(&output.stdout) {
            Ok(o) => program_result = o,
            Err(_) => return Err("Cannot decode output".into()),
        };

        grader_result.push_str(program_result);
        grader_result.push(',');
    }

    return Ok(TestResult {
        error: false,
        message: grader_result,
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_tests])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
