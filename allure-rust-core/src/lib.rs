use chrono::Utc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::thread::Result;
use uuid::Uuid;

pub mod attachment;
pub mod models;
pub mod writer;

pub use allure_rust_macros::allure_suite;
pub use allure_rust_macros::allure_test;
pub use allure_rust_macros::step;
pub use attachment::{AttachmentType, IntoAttachment};
pub use serde_json::json;

#[macro_export]
macro_rules! allure_step {
    ($title:expr, $body:block) => {{
        allure_rust::start_step($title);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $body));

        let step_result: std::thread::Result<()> = match &result {
            Ok(_) => Ok(()),
            Err(e) => {
                let cloned: Box<dyn std::any::Any + Send> =
                    if let Some(s) = e.downcast_ref::<&'static str>() {
                        Box::new(*s)
                    } else if let Some(s) = e.downcast_ref::<String>() {
                        Box::new(s.clone())
                    } else {
                        Box::new("Step failed")
                    };
                Err(cloned)
            }
        };
        allure_rust::end_step(&step_result);

        if result.is_err() {
            std::panic::resume_unwind(result.unwrap_err());
        }
    }};
}

struct TestContext {
    uuid: Uuid,
    steps: VecDeque<models::TestStep>,
    attachments: Vec<models::Attachment>,
    suite: Option<String>,
}

impl TestContext {
    fn new() -> Self {
        TestContext {
            uuid: Uuid::new_v4(),
            steps: VecDeque::new(),
            attachments: Vec::new(),
            suite: None,
        }
    }
}

// Thread-local storage for the test context
thread_local!(static TEST_CONTEXT: RefCell<TestContext> = RefCell::new(TestContext::new()));

pub fn start_test(#[allow(unused_variables)] name: &'static str) {
    start_test_with_context(name, None, None);
}

pub fn start_test_with_suite(
    #[allow(unused_variables)] name: &'static str,
    suite: Option<&'static str>,
) {
    start_test_with_context(name, suite, None);
}

pub fn start_test_with_context(
    #[allow(unused_variables)] name: &'static str,
    suite: Option<&'static str>,
    module_path: Option<&'static str>,
) {
    TEST_CONTEXT.with(|ctx| {
        let mut context = ctx.borrow_mut();
        *context = TestContext::new();

        if let Some(suite_name) = suite {
            context.suite = Some(suite_name.to_string());
        } else if let Some(path) = module_path {
            let suite_name = path.replace("::", ".");
            context.suite = Some(suite_name);
        }
    });
}

pub fn end_test(name: &'static str, result: Result<()>) {
    TEST_CONTEXT.with(|ctx| {
        let context = ctx.borrow();
        let stop_time = Utc::now().timestamp_millis();
        let (status, status_details) = match result {
            Ok(_) => (models::Status::Passed, None),
            Err(e) => {
                let panic_message = if let Some(s) = e.downcast_ref::<&'static str>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Test panicked".to_string()
                };
                (
                    models::Status::Failed,
                    Some(models::StatusDetails {
                        message: Some(panic_message),
                        trace: None, // You could add trace capturing here
                    }),
                )
            }
        };

        let mut labels = vec![];
        if let Some(suite_name) = &context.suite {
            labels.push(models::Label {
                name: "suite".to_string(),
                value: suite_name.clone(),
            });
        }

        let test_result = models::TestResult {
            uuid: context.uuid,
            history_id: Uuid::new_v4(),
            name: name.to_string(),
            description: None,
            status,
            status_details,
            stage: "finished".to_string(),
            start: stop_time - 1,
            stop: stop_time,
            labels,
            parameters: vec![],
            links: vec![],
            steps: context.steps.clone().into_iter().collect(),
            attachments: context.attachments.clone(),
        };

        writer::write_test_result(&test_result);
    });
}

pub fn start_step(name: &'static str) {
    start_step_with_params(name, Vec::new());
}

pub fn start_step_with_params(name: &'static str, parameters: Vec<models::Parameter>) {
    TEST_CONTEXT.with(|ctx| {
        let mut context = ctx.borrow_mut();
        let new_step = models::TestStep {
            name: name.to_string(),
            status: models::Status::Passed,
            status_details: None,
            stage: "running".to_string(),
            start: Utc::now().timestamp_millis(),
            stop: 0,
            steps: Vec::new(),
            attachments: Vec::new(),
            parameters,
        };
        context.steps.push_back(new_step);
    });
}

pub fn end_step(result: &Result<()>) {
    TEST_CONTEXT.with(|ctx| {
        let mut context = ctx.borrow_mut();
        if let Some(mut step) = context.steps.pop_back() {
            step.stop = Utc::now().timestamp_millis();
            step.stage = "finished".to_string();
            if let Err(e) = result {
                step.status = models::Status::Failed;
                let panic_message = if let Some(s) = e.downcast_ref::<&'static str>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Step panicked".to_string()
                };
                step.status_details = Some(models::StatusDetails {
                    message: Some(panic_message),
                    trace: None,
                });
            }

            if step.stage == "finished" && context.steps.is_empty() {
                context.steps.push_front(step);
            } else if let Some(parent_step) = context.steps.back_mut() {
                if parent_step.stage == "running" {
                    parent_step.steps.push(step);
                } else {
                    context.steps.push_front(step);
                }
            } else {
                context.steps.push_front(step);
            }
        }
    });
}

pub fn add_attachment<T: IntoAttachment>(name: impl Into<String>, content: T) {
    let attachment_type = content.attachment_type();
    let bytes = content.into_bytes();
    let source = writer::write_attachment(&bytes, attachment_type.extension());
    let attachment = models::Attachment {
        name: name.into(),
        source,
        attachment_type: attachment_type.mime_type().to_string(),
    };
    TEST_CONTEXT.with(|ctx| ctx.borrow_mut().attachments.push(attachment));
}

pub fn add_attachment_with_type<T: IntoAttachment>(
    name: impl Into<String>,
    content: T,
    attachment_type: AttachmentType,
) {
    let bytes = content.into_bytes();
    let source = writer::write_attachment(&bytes, attachment_type.extension());
    let attachment = models::Attachment {
        name: name.into(),
        source,
        attachment_type: attachment_type.mime_type().to_string(),
    };
    TEST_CONTEXT.with(|ctx| ctx.borrow_mut().attachments.push(attachment));
}
