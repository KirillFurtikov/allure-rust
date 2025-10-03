use allure_rust::{add_attachment, allure_test, json};

#[allure_test("Test with passed status")]
#[test]
fn test_passed() {
    let result = 2 + 2;
    assert_eq!(result, 4);

    add_attachment(
        "result",
        json!({
            "status": "passed",
            "message": "Test completed successfully"
        }),
    );
}

#[allure_test("Test with failed status")]
#[test]
fn test_failed() {
    add_attachment(
        "info",
        json!({
            "status": "failed",
            "reason": "Intentional failure for demonstration"
        }),
    );

    assert_eq!(2 + 2, 5, "This assertion will fail");
}

#[allure_test("Test with error status (panic)")]
#[test]
fn test_error() {
    add_attachment("error_info", "This test will panic");

    panic!("Intentional panic to demonstrate error status");
}

#[allure_test("Test with multiple assertions")]
#[test]
fn test_multiple_assertions() {
    assert!(true, "First assertion");
    assert_eq!(1 + 1, 2, "Second assertion");
    assert_ne!(1, 2, "Third assertion");

    add_attachment(
        "assertions_summary",
        json!({
            "total": 3,
            "passed": 3,
            "status": "all passed"
        }),
    );
}

#[allure_test("Test with conditional failure")]
#[test]
fn test_conditional() {
    let condition = false;

    add_attachment(
        "condition",
        json!({
            "value": condition,
            "expected": true
        }),
    );

    assert!(condition, "Condition should be true");
}
