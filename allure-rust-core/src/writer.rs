use crate::models::TestResult;
use std::env;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

const DEFAULT_RESULTS_DIR: &str = "allure-results";

fn get_results_dir() -> PathBuf {
    env::var("ALLURE_RESULTS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_RESULTS_DIR))
}

pub fn write_test_result(test_result: &TestResult) {
    let dir = get_results_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir).expect("Failed to create allure-results directory");
    }
    let filename = dir.join(format!("{}-result.json", test_result.uuid));
    let json = serde_json::to_string_pretty(test_result).expect("Failed to serialize TestResult");
    fs::write(filename, json).expect("Failed to write TestResult to file");
}

pub fn write_attachment(source: &[u8], extension: &str) -> String {
    let dir = get_results_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir).expect("Failed to create allure-results directory");
    }
    let filename = format!("{}.{}", Uuid::new_v4(), extension);
    let filepath = dir.join(&filename);
    fs::write(filepath, source).expect("Failed to write attachment file");
    filename
}
