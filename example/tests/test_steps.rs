#[cfg(test)]
mod tests {

    use allure_rust::{add_attachment, allure_test, json, step};
    #[step("Step that passes")]
    fn step_passed(value: i32) -> i32 {
        println!("Processing value: {}", value);
        value * 2
    }

    #[step("Step that fails with assertion")]
    fn step_failed(value: i32) -> i32 {
        assert!(value > 10, "Value must be greater than 10");
        value
    }

    #[step("Step that panics")]
    fn step_error(value: i32) -> i32 {
        if value == 0 {
            panic!("Value cannot be zero!");
        }
        value
    }

    #[step("Nested step - level 1")]
    fn nested_step_level1(a: i32, b: i32) -> i32 {
        let result = nested_step_level2(a, b);
        result + 10
    }

    #[step("Nested step - level 2")]
    fn nested_step_level2(a: i32, b: i32) -> i32 {
        a + b
    }

    #[step("Step with multiple parameters")]
    fn step_with_params(name: &str, age: i32, active: bool) -> String {
        format!("User: {}, Age: {}, Active: {}", name, age, active)
    }

    #[step("Step with attachment")]
    fn step_with_attachment(data: &str) {
        add_attachment("step_data", data);
    }

    #[allure_test("Test with all steps passing")]
    #[test]
    fn test_all_steps_pass() {
        let result1 = step_passed(5);
        assert_eq!(result1, 10);

        let result2 = step_passed(result1);
        assert_eq!(result2, 20);

        add_attachment(
            "summary",
            json!({
                "steps": 2,
                "status": "all passed"
            }),
        );
    }

    #[allure_test("Test with failed step")]
    #[test]
    fn test_step_fails() {
        let value = 5;
        step_failed(value);
    }

    #[allure_test("Test with error in step")]
    #[test]
    fn test_step_error() {
        step_error(0);
    }

    #[allure_test("Test with nested steps")]
    #[test]
    fn test_nested_steps() {
        let result = nested_step_level1(10, 20);
        assert_eq!(result, 40);

        add_attachment(
            "nested_result",
            json!({
                "final_result": result,
                "levels": 2
            }),
        );
    }

    #[allure_test("Test with step parameters")]
    #[test]
    fn test_step_parameters() {
        let user1 = step_with_params("Alice", 30, true);
        let user2 = step_with_params("Bob", 25, false);

        add_attachment(
            "users",
            json!({
                "user1": user1,
                "user2": user2
            }),
        );
    }

    #[allure_test("Test with step attachments")]
    #[test]
    fn test_step_attachments() {
        step_with_attachment("First data");
        step_with_attachment("Second data");
        step_with_attachment("Third data");
    }

    #[allure_test("Test with mixed step statuses")]
    #[test]
    fn test_mixed_steps() {
        let result1 = step_passed(10);
        assert_eq!(result1, 20);

        step_failed(5);
    }

    #[allure_test("Test with complex step flow")]
    #[test]
    fn test_complex_flow() {
        let step1 = step_passed(5);
        let step2 = nested_step_level1(step1, 15);
        let user = step_with_params("Charlie", 35, true);
        step_with_attachment(&user);

        add_attachment(
            "flow_summary",
            json!({
                "step1_result": step1,
                "step2_result": step2,
                "user": user
            }),
        );
    }
}
