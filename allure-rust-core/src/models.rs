use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Passed,
    Failed,
    Broken,
    Skipped,
}

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatusDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub name: String,
    pub source: String, // filename
    #[serde(rename = "type")]
    pub attachment_type: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TestStep {
    pub name: String,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<StatusDetails>,
    pub stage: String,
    pub start: i64,
    pub stop: i64,
    #[serde(default)]
    pub steps: Vec<TestStep>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TestResult {
    #[serde(default = "Uuid::new_v4")]
    pub uuid: Uuid,
    #[serde(default = "Uuid::new_v4")]
    pub history_id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<StatusDetails>,
    pub stage: String,
    pub start: i64,
    pub stop: i64,
    #[serde(default)]
    pub labels: Vec<Label>,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
    #[serde(default)]
    pub links: Vec<Link>,
    #[serde(default)]
    pub steps: Vec<TestStep>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Label {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct Link {
    pub name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub link_type: String,
}
