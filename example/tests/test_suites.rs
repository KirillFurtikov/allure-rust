use allure_rust::{add_attachment, allure_suite, allure_test, json};

#[allure_suite("Calculator Suite")]
mod calculator_tests {
    use super::*;

    #[allure_test("Addition test")]
    #[test]
    fn test_add() {
        let result = 2 + 2;
        assert_eq!(result, 4);

        add_attachment(
            "calculation",
            json!({
                "operation": "addition",
                "result": result
            }),
        );
    }

    #[allure_test("Subtraction test")]
    #[test]
    fn test_subtract() {
        let result = 10 - 5;
        assert_eq!(result, 5);

        add_attachment(
            "calculation",
            json!({
                "operation": "subtraction",
                "result": result
            }),
        );
    }

    #[allure_test("Multiplication test")]
    #[test]
    fn test_multiply() {
        let result = 3 * 4;
        assert_eq!(result, 12);
    }
}

#[allure_suite("String Operations Suite")]
mod string_tests {
    use super::*;

    #[allure_test("Concatenation test")]
    #[test]
    fn test_concat() {
        let result = format!("{} {}", "Hello", "World");
        assert_eq!(result, "Hello World");

        add_attachment("result", result);
    }

    #[allure_test("Uppercase test")]
    #[test]
    fn test_uppercase() {
        let result = "hello".to_uppercase();
        assert_eq!(result, "HELLO");
    }

    #[allure_test("Length test")]
    #[test]
    fn test_length() {
        let text = "Hello, World!";
        assert_eq!(text.len(), 13);

        add_attachment(
            "text_info",
            json!({
                "text": text,
                "length": text.len()
            }),
        );
    }
}

#[allure_suite("Collection Operations Suite")]
mod collection_tests {
    use super::*;

    #[allure_test("Vector push test")]
    #[test]
    fn test_vec_push() {
        let mut vec = vec![1, 2, 3];
        vec.push(4);
        assert_eq!(vec.len(), 4);
        assert_eq!(vec[3], 4);
    }

    #[allure_test("Vector filter test")]
    #[test]
    fn test_vec_filter() {
        let vec = vec![1, 2, 3, 4, 5, 6];
        let even: Vec<_> = vec.iter().filter(|&&x| x % 2 == 0).collect();
        assert_eq!(even.len(), 3);

        add_attachment(
            "filter_result",
            json!({
                "original": vec,
                "filtered": even
            }),
        );
    }

    #[allure_test("Vector map test")]
    #[test]
    fn test_vec_map() {
        let vec = vec![1, 2, 3];
        let doubled: Vec<_> = vec.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }
}

#[allure_suite("Error Handling Suite")]
mod error_tests {
    use super::*;

    #[allure_test("Division by zero")]
    #[test]
    fn test_division_by_zero() {
        add_attachment("info", "This test demonstrates error handling");
    }

    #[allure_test("Panic test")]
    #[test]
    fn test_panic() {
        panic!("Intentional panic in Error Handling Suite");
    }

    #[allure_test("Assertion failure")]
    #[test]
    fn test_assertion() {
        assert_eq!(1, 2, "Numbers should be equal");
    }
}

#[allure_suite("Integration Tests Suite")]
mod integration_tests {
    use super::*;

    #[allure_test("Complex workflow")]
    #[test]
    fn test_workflow() {
        let step1 = 10 + 5;
        let step2 = step1 * 2;
        let step3 = step2 - 10;

        assert_eq!(step3, 20);

        add_attachment(
            "workflow",
            json!({
                "step1": step1,
                "step2": step2,
                "step3": step3
            }),
        );
    }

    #[allure_test("Data transformation")]
    #[test]
    fn test_transformation() {
        let data = vec!["hello", "world", "rust"];
        let transformed: Vec<_> = data.iter().map(|s| s.to_uppercase()).collect();

        assert_eq!(transformed, vec!["HELLO", "WORLD", "RUST"]);

        add_attachment(
            "transformation",
            json!({
                "original": data,
                "transformed": transformed
            }),
        );
    }
}
