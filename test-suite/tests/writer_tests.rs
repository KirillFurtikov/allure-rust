use allure_rust_core::models::{Status, TestResult};
use allure_rust_core::writer::{write_attachment, write_test_result};
use serial_test::serial;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use uuid::Uuid;

#[test]
#[serial]
fn test_write_test_result() {
    let temp_dir = TempDir::new().unwrap();
    let results_dir = temp_dir.path().to_str().unwrap();

    std::env::set_var("ALLURE_RESULTS_DIR", results_dir);

    let test_result = TestResult {
        uuid: Uuid::new_v4(),
        history_id: Uuid::new_v4(),
        name: "Test write result".to_string(),
        description: None,
        status: Status::Passed,
        status_details: None,
        stage: "finished".to_string(),
        start: 1000,
        stop: 2000,
        labels: vec![],
        parameters: vec![],
        links: vec![],
        steps: vec![],
        attachments: vec![],
    };

    write_test_result(&test_result);

    let expected_file =
        PathBuf::from(results_dir).join(format!("{}-result.json", test_result.uuid));
    assert!(expected_file.exists());

    let content = fs::read_to_string(&expected_file).unwrap();
    assert!(content.contains("\"name\""));
    assert!(content.contains("Test write result"));
    assert!(content.contains("\"status\""));
    assert!(content.contains("passed"));
}

#[test]
#[serial]
fn test_write_attachment_text() {
    let temp_dir = TempDir::new().unwrap();
    let results_dir = temp_dir.path().to_str().unwrap();

    std::env::set_var("ALLURE_RESULTS_DIR", results_dir);

    let content = b"Test attachment content";
    let filename = write_attachment(content, "txt");

    let file_path = PathBuf::from(results_dir).join(&filename);
    assert!(file_path.exists());

    let saved_content = fs::read(&file_path).unwrap();
    assert_eq!(saved_content, content);
}

#[test]
#[serial]
fn test_write_attachment_json() {
    let temp_dir = TempDir::new().unwrap();
    let results_dir = temp_dir.path().to_str().unwrap();

    std::env::set_var("ALLURE_RESULTS_DIR", results_dir);

    let json_content = br#"{"key": "value"}"#;
    let filename = write_attachment(json_content, "json");

    let file_path = PathBuf::from(results_dir).join(&filename);
    assert!(file_path.exists());
    assert!(filename.ends_with(".json"));

    let saved_content = fs::read_to_string(&file_path).unwrap();
    assert!(saved_content.contains("\"key\""));
}

#[test]
#[serial]
fn test_write_attachment_binary() {
    let temp_dir = TempDir::new().unwrap();
    let results_dir = temp_dir.path().to_str().unwrap();

    std::env::set_var("ALLURE_RESULTS_DIR", results_dir);

    let binary_data = vec![0u8, 1, 2, 3, 4, 5, 255];
    let filename = write_attachment(&binary_data, "png");

    let file_path = PathBuf::from(results_dir).join(&filename);
    assert!(file_path.exists());
    assert!(filename.ends_with(".png"));

    let saved_content = fs::read(&file_path).unwrap();
    assert_eq!(saved_content, binary_data);
}

#[test]
#[serial]
fn test_results_directory_creation() {
    let temp_dir = TempDir::new().unwrap();
    let results_dir = temp_dir.path().join("nested").join("allure-results");
    let results_path = results_dir.to_str().unwrap();

    std::env::set_var("ALLURE_RESULTS_DIR", results_path);

    assert!(!results_dir.exists());

    let test_result = TestResult {
        uuid: Uuid::new_v4(),
        history_id: Uuid::new_v4(),
        name: "Test directory creation".to_string(),
        description: None,
        status: Status::Passed,
        status_details: None,
        stage: "finished".to_string(),
        start: 1000,
        stop: 2000,
        labels: vec![],
        parameters: vec![],
        links: vec![],
        steps: vec![],
        attachments: vec![],
    };

    write_test_result(&test_result);

    assert!(results_dir.exists());
    assert!(results_dir.is_dir());
}

#[test]
#[serial]
fn test_multiple_test_results() {
    let temp_dir = TempDir::new().unwrap();
    let results_dir = temp_dir.path().to_str().unwrap();

    std::env::set_var("ALLURE_RESULTS_DIR", results_dir);

    for i in 0..5 {
        let test_result = TestResult {
            uuid: Uuid::new_v4(),
            history_id: Uuid::new_v4(),
            name: format!("Test {}", i),
            description: None,
            status: Status::Passed,
            status_details: None,
            stage: "finished".to_string(),
            start: 1000,
            stop: 2000,
            labels: vec![],
            parameters: vec![],
            links: vec![],
            steps: vec![],
            attachments: vec![],
        };

        write_test_result(&test_result);
    }

    let files: Vec<_> = fs::read_dir(results_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert_eq!(files.len(), 5);
}

#[test]
#[serial]
fn test_attachment_filename_uniqueness() {
    let temp_dir = TempDir::new().unwrap();
    let results_dir = temp_dir.path().to_str().unwrap();

    std::env::set_var("ALLURE_RESULTS_DIR", results_dir);

    let filename1 = write_attachment(b"content1", "txt");
    let filename2 = write_attachment(b"content2", "txt");

    assert_ne!(filename1, filename2);

    let file1 = PathBuf::from(results_dir).join(&filename1);
    let file2 = PathBuf::from(results_dir).join(&filename2);

    assert!(file1.exists());
    assert!(file2.exists());

    let content1 = fs::read_to_string(&file1).unwrap();
    let content2 = fs::read_to_string(&file2).unwrap();

    assert_eq!(content1, "content1");
    assert_eq!(content2, "content2");
}
