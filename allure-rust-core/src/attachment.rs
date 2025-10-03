use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Copy)]
pub enum AttachmentType {
    Text,
    Html,
    Xml,
    Json,
    Yaml,
    Csv,
    Tsv,
    UriList,
    Png,
    Jpeg,
    Gif,
    Bmp,
    Tiff,
    Svg,
    ImageDiff,
    Mp4,
    Ogg,
    Webm,
}

impl AttachmentType {
    pub fn mime_type(&self) -> &'static str {
        match self {
            AttachmentType::Text => "text/plain",
            AttachmentType::Html => "text/html",
            AttachmentType::Xml => "application/xml",
            AttachmentType::Json => "application/json",
            AttachmentType::Yaml => "application/yaml",
            AttachmentType::Csv => "text/csv",
            AttachmentType::Tsv => "text/tab-separated-values",
            AttachmentType::UriList => "text/uri-list",
            AttachmentType::Png => "image/png",
            AttachmentType::Jpeg => "image/jpeg",
            AttachmentType::Gif => "image/gif",
            AttachmentType::Bmp => "image/bmp",
            AttachmentType::Tiff => "image/tiff",
            AttachmentType::Svg => "image/svg+xml",
            AttachmentType::ImageDiff => "application/vnd.allure.image.diff",
            AttachmentType::Mp4 => "video/mp4",
            AttachmentType::Ogg => "video/ogg",
            AttachmentType::Webm => "video/webm",
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            AttachmentType::Text => "txt",
            AttachmentType::Html => "html",
            AttachmentType::Xml => "xml",
            AttachmentType::Json => "json",
            AttachmentType::Yaml => "yaml",
            AttachmentType::Csv => "csv",
            AttachmentType::Tsv => "tsv",
            AttachmentType::UriList => "uri",
            AttachmentType::Png => "png",
            AttachmentType::Jpeg => "jpg",
            AttachmentType::Gif => "gif",
            AttachmentType::Bmp => "bmp",
            AttachmentType::Tiff => "tiff",
            AttachmentType::Svg => "svg",
            AttachmentType::ImageDiff => "diff.png",
            AttachmentType::Mp4 => "mp4",
            AttachmentType::Ogg => "ogg",
            AttachmentType::Webm => "webm",
        }
    }
}

pub trait IntoAttachment {
    fn into_bytes(self) -> Vec<u8>;
    fn attachment_type(&self) -> AttachmentType;
}

impl IntoAttachment for &str {
    fn into_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn attachment_type(&self) -> AttachmentType {
        AttachmentType::Text
    }
}

impl IntoAttachment for String {
    fn into_bytes(self) -> Vec<u8> {
        self.into_bytes()
    }

    fn attachment_type(&self) -> AttachmentType {
        AttachmentType::Text
    }
}

impl IntoAttachment for &[u8] {
    fn into_bytes(self) -> Vec<u8> {
        self.to_vec()
    }

    fn attachment_type(&self) -> AttachmentType {
        AttachmentType::Text
    }
}

impl IntoAttachment for Vec<u8> {
    fn into_bytes(self) -> Vec<u8> {
        self
    }

    fn attachment_type(&self) -> AttachmentType {
        AttachmentType::Text
    }
}

impl IntoAttachment for JsonValue {
    fn into_bytes(self) -> Vec<u8> {
        serde_json::to_string_pretty(&self)
            .unwrap_or_else(|_| "{}".to_string())
            .into_bytes()
    }

    fn attachment_type(&self) -> AttachmentType {
        AttachmentType::Json
    }
}

pub struct TypedAttachment<T> {
    pub content: T,
    pub attachment_type: AttachmentType,
}

impl<T: IntoAttachment> IntoAttachment for TypedAttachment<T> {
    fn into_bytes(self) -> Vec<u8> {
        self.content.into_bytes()
    }

    fn attachment_type(&self) -> AttachmentType {
        self.attachment_type
    }
}
