use allure_rust::{allure_step, allure_test};

#[allure_test("Test with assert steps")]
#[test]
fn test_assert_steps() {
    let value = 42;

    allure_step!("Check value is positive", {
        assert!(value > 0, "Value should be positive");
    });

    allure_step!("Check value is less than 100", {
        assert!(value < 100, "Value should be less than 100");
    });

    allure_step!("Check value equals 42", {
        assert_eq!(value, 42, "Value should equal 42");
    });

    allure_step!("Check value is not zero", {
        assert_ne!(value, 0, "Value should not be zero");
    });
}

#[allure_test("Test with failing assert step")]
#[test]
fn test_failing_assert() {
    let x = 5;
    let y = 10;

    allure_step!("Verify x equals 5", {
        assert_eq!(x, 5);
    });

    allure_step!("Verify y equals 10", {
        assert_eq!(y, 10);
    });

    allure_step!("Verify sum equals 20", {
        assert_eq!(x + y, 20, "Sum should be 20");
    });
}

#[allure_test("Test with multiple assertions")]
#[test]
fn test_multiple_assertions() {
    let numbers = vec![1, 2, 3, 4, 5];

    allure_step!("Verify vector is not empty", {
        assert!(!numbers.is_empty(), "Vector should not be empty");
    });

    allure_step!("Verify vector length", {
        assert_eq!(numbers.len(), 5, "Vector should have 5 elements");
    });

    allure_step!("Calculate and verify sum", {
        let sum: i32 = numbers.iter().sum();
        assert_eq!(sum, 15, "Sum should be 15");
    });

    allure_step!("Verify first element", {
        let first = numbers.first().unwrap();
        assert_eq!(*first, 1, "First element should be 1");
    });

    allure_step!("Verify last element", {
        let last = numbers.last().unwrap();
        assert_eq!(*last, 5, "Last element should be 5");
    });
}

#[allure_test("Test with string assertions")]
#[test]
fn test_string_assertions() {
    let text = "Hello, World!";

    allure_step!("Check text contains Hello", {
        assert!(text.contains("Hello"), "Text should contain 'Hello'");
    });

    allure_step!("Check text contains World", {
        assert!(text.contains("World"), "Text should contain 'World'");
    });

    allure_step!("Verify text length", {
        assert_eq!(text.len(), 13, "Text length should be 13");
    });

    allure_step!("Verify text is not Goodbye", {
        assert_ne!(text, "Goodbye", "Text should not be 'Goodbye'");
    });
}

#[allure_test("Test with boolean assertions")]
#[test]
fn test_boolean_assertions() {
    let is_valid = true;
    let is_empty = false;

    allure_step!("Check is_valid", {
        assert!(is_valid, "Should be valid");
    });

    allure_step!("Check is_empty", {
        assert!(!is_empty, "Should not be empty");
    });

    allure_step!("Verify is_valid equals true", {
        assert_eq!(is_valid, true);
    });

    allure_step!("Verify is_empty not equals true", {
        assert_ne!(is_empty, true);
    });
}

#[allure_test("Test with nested steps")]
#[test]
fn test_nested_steps() {
    allure_step!("Calculate sum", {
        let a = 10;
        let b = 20;
        let sum = a + b;

        allure_step!("Verify sum equals 30", {
            assert_eq!(sum, 30, "Sum should be 30");
        });
    });

    allure_step!("Verify string", {
        let text = "test";

        allure_step!("Check length", {
            assert!(text.len() > 0);
        });

        allure_step!("Check value", {
            assert_eq!(text, "test");
        });
    });
}
