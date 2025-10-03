use allure_rust::{add_attachment, allure_step, allure_test, json, step};
use serial_test::serial;

#[step("Context test step")]
fn context_step(value: i32) -> i32 {
    value * 2
}

#[allure_test("Test context isolation")]
#[test]
#[serial]
fn test_context_isolation() {
    allure_step!("First step", {
        let x = 10;
        assert_eq!(x, 10);
    });

    allure_step!("Second step", {
        let y = 20;
        assert_eq!(y, 20);
    });
}

#[allure_test("Test context with attachments")]
#[test]
#[serial]
fn test_context_attachments() {
    add_attachment("attachment1", "First attachment");
    add_attachment("attachment2", "Second attachment");

    add_attachment(
        "json_attachment",
        json!({
            "key": "value"
        }),
    );

    assert!(true);
}

#[allure_test("Test context with steps and attachments")]
#[test]
#[serial]
fn test_context_steps_and_attachments() {
    allure_step!("Step with attachment", {
        add_attachment("step_data", "Data from step");
        assert!(true);
    });

    add_attachment("test_data", "Data from test");
}

#[allure_test("Test context with function steps")]
#[test]
#[serial]
fn test_context_function_steps() {
    let result1 = context_step(5);
    assert_eq!(result1, 10);

    let result2 = context_step(result1);
    assert_eq!(result2, 20);
}

#[allure_test("Test context with nested steps")]
#[test]
#[serial]
fn test_context_nested_steps() {
    allure_step!("Outer step", {
        add_attachment("outer_data", "Outer step data");

        allure_step!("Inner step 1", {
            add_attachment("inner1_data", "Inner step 1 data");
            assert!(true);
        });

        allure_step!("Inner step 2", {
            add_attachment("inner2_data", "Inner step 2 data");
            assert_eq!(1 + 1, 2);
        });
    });
}

#[allure_test("Test context step parameters")]
#[test]
#[serial]
fn test_context_step_parameters() {
    #[step("Step with params")]
    fn step_with_params(a: i32, b: i32, name: &str) -> i32 {
        println!("Processing {} with {} and {}", name, a, b);
        a + b
    }

    let result = step_with_params(10, 20, "test");
    assert_eq!(result, 30);
}

#[allure_test("Test context multiple attachments")]
#[test]
#[serial]
fn test_context_multiple_attachments() {
    for i in 0..5 {
        add_attachment(
            format!("attachment_{}", i),
            json!({
                "index": i,
                "value": i * 10
            }),
        );
    }

    assert!(true);
}

#[allure_test("Test context step status")]
#[test]
#[serial]
fn test_context_step_status() {
    allure_step!("Passing step", {
        assert_eq!(2 + 2, 4);
    });

    allure_step!("Another passing step", {
        assert!(true);
    });
}

#[allure_test("Test context sequential steps")]
#[test]
#[serial]
fn test_context_sequential_steps() {
    allure_step!("Step 1", {
        add_attachment("step1", "Step 1 data");
    });

    allure_step!("Step 2", {
        add_attachment("step2", "Step 2 data");
    });

    allure_step!("Step 3", {
        add_attachment("step3", "Step 3 data");
    });

    allure_step!("Step 4", {
        add_attachment("step4", "Step 4 data");
    });
}

#[allure_test("Test context with complex data")]
#[test]
#[serial]
fn test_context_complex_data() {
    allure_step!("Process complex data", {
        let data = json!({
            "users": [
                {"name": "Alice", "age": 30},
                {"name": "Bob", "age": 25}
            ],
            "metadata": {
                "version": "1.0",
                "timestamp": 1234567890
            }
        });

        add_attachment("complex_data", data);
        assert!(true);
    });
}
