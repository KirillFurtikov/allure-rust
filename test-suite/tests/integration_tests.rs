use allure_rust::{add_attachment, allure_step, allure_test, json, step};

#[step("Helper step function")]
fn helper_step(value: i32) -> i32 {
    value * 2
}

#[allure_test("Integration test - basic functionality")]
#[test]
fn test_basic_integration() {
    let result = helper_step(21);
    assert_eq!(result, 42);
}

#[allure_test("Integration test - with inline steps")]
#[test]
fn test_with_inline_steps() {
    allure_step!("Step 1: Initialize", {
        let x = 10;
        assert_eq!(x, 10);
    });

    allure_step!("Step 2: Process", {
        let y = 20;
        assert_eq!(y, 20);
    });

    allure_step!("Step 3: Verify", {
        assert!(true);
    });
}

#[allure_test("Integration test - with attachments")]
#[test]
fn test_with_attachments() {
    add_attachment("text_data", "Sample text");

    add_attachment(
        "json_data",
        json!({
            "test": "integration",
            "status": "running"
        }),
    );

    assert!(true);
}

#[allure_test("Integration test - nested steps")]
#[test]
fn test_nested_steps_integration() {
    allure_step!("Outer step", {
        let outer_value = 100;

        allure_step!("Inner step 1", {
            assert!(outer_value > 0);
        });

        allure_step!("Inner step 2", {
            assert_eq!(outer_value, 100);
        });
    });
}

#[allure_test("Integration test - function steps with parameters")]
#[test]
fn test_function_steps_with_params() {
    #[step("Calculate sum")]
    fn calculate(a: i32, b: i32) -> i32 {
        a + b
    }

    #[step("Verify result")]
    fn verify(actual: i32, expected: i32) {
        assert_eq!(actual, expected);
    }

    let result = calculate(10, 20);
    verify(result, 30);
}

#[allure_test("Integration test - mixed steps and attachments")]
#[test]
fn test_mixed_functionality() {
    allure_step!("Setup phase", {
        add_attachment("setup_info", "Initializing test data");
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(data.len(), 5);
    });

    allure_step!("Execution phase", {
        let result = helper_step(5);
        add_attachment(
            "execution_result",
            json!({
                "input": 5,
                "output": result
            }),
        );
        assert_eq!(result, 10);
    });

    allure_step!("Verification phase", {
        add_attachment("verification_status", "All checks passed");
        assert!(true);
    });
}

#[allure_test("Integration test - error handling")]
#[test]
fn test_error_handling() {
    allure_step!("Step 1", {
        add_attachment("info", "First step completed");
        assert!(true);
    });

    allure_step!("Step 2", {
        add_attachment("info", "Second step completed");
        assert_eq!(1 + 1, 2);
    });
}

#[allure_test("Integration test - multiple function steps")]
#[test]
fn test_multiple_function_steps() {
    #[step("Step 1")]
    fn step1() -> i32 {
        42
    }

    #[step("Step 2")]
    fn step2(value: i32) -> i32 {
        value + 10
    }

    #[step("Step 3")]
    fn step3(value: i32) -> String {
        format!("Result: {}", value)
    }

    let val1 = step1();
    let val2 = step2(val1);
    let result = step3(val2);

    assert_eq!(result, "Result: 52");
}

#[allure_test("Integration test - complex nested structure")]
#[test]
fn test_complex_nested_structure() {
    allure_step!("Level 1", {
        add_attachment("level1_info", "Starting level 1");

        allure_step!("Level 2a", {
            add_attachment("level2a_info", "In level 2a");

            allure_step!("Level 3", {
                assert!(true);
            });
        });

        allure_step!("Level 2b", {
            add_attachment("level2b_info", "In level 2b");
            assert_eq!(1 + 1, 2);
        });
    });
}

#[allure_test("Integration test - data-driven")]
#[test]
fn test_data_driven() {
    let test_cases = vec![(2, 4), (5, 10), (10, 20), (100, 200)];

    for (input, expected) in test_cases {
        allure_step!("Execute test case", {
            let result = helper_step(input);
            add_attachment(
                "test_case_result",
                json!({
                    "input": input,
                    "expected": expected,
                    "actual": result
                }),
            );
            assert_eq!(result, expected);
        });
    }
}
