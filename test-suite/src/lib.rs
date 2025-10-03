#[cfg(test)]
mod tests {
    use allure_rust_core::models::*;
    use serde_json;

    #[test]
    fn test_status_serialization() {
        let status = Status::Passed;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"passed\"");

        let status = Status::Failed;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"failed\"");

        let status = Status::Broken;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"broken\"");

        let status = Status::Skipped;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"skipped\"");
    }

    #[test]
    fn test_label_creation() {
        let label = Label {
            name: "suite".to_string(),
            value: "Test Suite".to_string(),
        };

        assert_eq!(label.name, "suite");
        assert_eq!(label.value, "Test Suite");
    }

    #[test]
    fn test_parameter_creation() {
        let param = Parameter {
            name: "username".to_string(),
            value: "john_doe".to_string(),
        };

        assert_eq!(param.name, "username");
        assert_eq!(param.value, "john_doe");
    }

    #[test]
    fn test_status_details_with_message() {
        let details = StatusDetails {
            message: Some("Test failed".to_string()),
            trace: None,
        };

        assert_eq!(details.message, Some("Test failed".to_string()));
        assert_eq!(details.trace, None);
    }

    #[test]
    fn test_status_details_with_trace() {
        let details = StatusDetails {
            message: Some("Error occurred".to_string()),
            trace: Some("Stack trace here".to_string()),
        };

        assert_eq!(details.message, Some("Error occurred".to_string()));
        assert_eq!(details.trace, Some("Stack trace here".to_string()));
    }

    #[test]
    fn test_attachment_creation() {
        let attachment = Attachment {
            name: "screenshot".to_string(),
            source: "screenshot.png".to_string(),
            attachment_type: "image/png".to_string(),
        };

        assert_eq!(attachment.name, "screenshot");
        assert_eq!(attachment.source, "screenshot.png");
        assert_eq!(attachment.attachment_type, "image/png");
    }

    #[test]
    fn test_test_step_creation() {
        let step = TestStep {
            name: "Login step".to_string(),
            status: Status::Passed,
            status_details: None,
            stage: "finished".to_string(),
            start: 1000,
            stop: 2000,
            steps: vec![],
            attachments: vec![],
            parameters: vec![],
        };

        assert_eq!(step.name, "Login step");
        assert!(matches!(step.status, Status::Passed));
        assert_eq!(step.stage, "finished");
        assert_eq!(step.start, 1000);
        assert_eq!(step.stop, 2000);
    }

    #[test]
    fn test_nested_steps() {
        let inner_step = TestStep {
            name: "Inner step".to_string(),
            status: Status::Passed,
            status_details: None,
            stage: "finished".to_string(),
            start: 1500,
            stop: 1800,
            steps: vec![],
            attachments: vec![],
            parameters: vec![],
        };

        let outer_step = TestStep {
            name: "Outer step".to_string(),
            status: Status::Passed,
            status_details: None,
            stage: "finished".to_string(),
            start: 1000,
            stop: 2000,
            steps: vec![inner_step],
            attachments: vec![],
            parameters: vec![],
        };

        assert_eq!(outer_step.steps.len(), 1);
        assert_eq!(outer_step.steps[0].name, "Inner step");
    }

    #[test]
    fn test_test_result_serialization() {
        use uuid::Uuid;

        let uuid = Uuid::new_v4();
        let history_id = Uuid::new_v4();

        let test_result = TestResult {
            uuid,
            history_id,
            name: "Test name".to_string(),
            description: Some("Test description".to_string()),
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

        let json = serde_json::to_string(&test_result).unwrap();
        assert!(json.contains("\"name\":\"Test name\""));
        assert!(json.contains("\"status\":\"passed\""));
        assert!(json.contains("\"stage\":\"finished\""));
    }

    #[test]
    fn test_test_result_with_labels() {
        use uuid::Uuid;

        let test_result = TestResult {
            uuid: Uuid::new_v4(),
            history_id: Uuid::new_v4(),
            name: "Test with labels".to_string(),
            description: None,
            status: Status::Passed,
            status_details: None,
            stage: "finished".to_string(),
            start: 1000,
            stop: 2000,
            labels: vec![
                Label {
                    name: "suite".to_string(),
                    value: "My Suite".to_string(),
                },
                Label {
                    name: "feature".to_string(),
                    value: "Login".to_string(),
                },
            ],
            parameters: vec![],
            links: vec![],
            steps: vec![],
            attachments: vec![],
        };

        assert_eq!(test_result.labels.len(), 2);
        assert_eq!(test_result.labels[0].name, "suite");
        assert_eq!(test_result.labels[1].name, "feature");
    }

    #[test]
    fn test_test_result_with_parameters() {
        use uuid::Uuid;

        let test_result = TestResult {
            uuid: Uuid::new_v4(),
            history_id: Uuid::new_v4(),
            name: "Parameterized test".to_string(),
            description: None,
            status: Status::Passed,
            status_details: None,
            stage: "finished".to_string(),
            start: 1000,
            stop: 2000,
            labels: vec![],
            parameters: vec![
                Parameter {
                    name: "input".to_string(),
                    value: "42".to_string(),
                },
                Parameter {
                    name: "expected".to_string(),
                    value: "42".to_string(),
                },
            ],
            links: vec![],
            steps: vec![],
            attachments: vec![],
        };

        assert_eq!(test_result.parameters.len(), 2);
        assert_eq!(test_result.parameters[0].name, "input");
        assert_eq!(test_result.parameters[1].value, "42");
    }
}
