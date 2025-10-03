use allure_rust_core::attachment::{AttachmentType, IntoAttachment};

#[test]
fn test_attachment_type_text() {
    let att_type = AttachmentType::Text;
    assert_eq!(att_type.mime_type(), "text/plain");
    assert_eq!(att_type.extension(), "txt");
}

#[test]
fn test_attachment_type_json() {
    let att_type = AttachmentType::Json;
    assert_eq!(att_type.mime_type(), "application/json");
    assert_eq!(att_type.extension(), "json");
}

#[test]
fn test_attachment_type_html() {
    let att_type = AttachmentType::Html;
    assert_eq!(att_type.mime_type(), "text/html");
    assert_eq!(att_type.extension(), "html");
}

#[test]
fn test_attachment_type_xml() {
    let att_type = AttachmentType::Xml;
    assert_eq!(att_type.mime_type(), "application/xml");
    assert_eq!(att_type.extension(), "xml");
}

#[test]
fn test_attachment_type_png() {
    let att_type = AttachmentType::Png;
    assert_eq!(att_type.mime_type(), "image/png");
    assert_eq!(att_type.extension(), "png");
}

#[test]
fn test_attachment_type_jpeg() {
    let att_type = AttachmentType::Jpeg;
    assert_eq!(att_type.mime_type(), "image/jpeg");
    assert_eq!(att_type.extension(), "jpg");
}

#[test]
fn test_string_into_attachment() {
    let text = "Hello, World!";
    let bytes = text.into_bytes();
    assert_eq!(bytes, b"Hello, World!");

    let att_type = text.attachment_type();
    assert_eq!(att_type.mime_type(), "text/plain");
}

#[test]
fn test_string_owned_into_attachment() {
    let text = String::from("Test string");
    let att_type = text.attachment_type();
    assert_eq!(att_type.mime_type(), "text/plain");

    let bytes = text.into_bytes();
    assert_eq!(bytes, b"Test string");
}

#[test]
fn test_vec_u8_into_attachment() {
    let data = vec![1u8, 2, 3, 4, 5];
    let att_type = data.attachment_type();
    assert_eq!(att_type.mime_type(), "text/plain");

    let bytes = data.into_bytes();
    assert_eq!(bytes, vec![1u8, 2, 3, 4, 5]);
}

#[test]
fn test_json_value_into_attachment() {
    use serde_json::json;

    let value = json!({
        "name": "John",
        "age": 30
    });

    let att_type = value.attachment_type();
    assert_eq!(att_type.mime_type(), "application/json");

    let bytes = value.into_bytes();
    let json_str = String::from_utf8(bytes).unwrap();
    assert!(json_str.contains("\"name\""));
    assert!(json_str.contains("\"John\""));
}

#[test]
fn test_all_image_types() {
    let types = vec![
        (AttachmentType::Png, "image/png", "png"),
        (AttachmentType::Jpeg, "image/jpeg", "jpg"),
        (AttachmentType::Gif, "image/gif", "gif"),
        (AttachmentType::Bmp, "image/bmp", "bmp"),
        (AttachmentType::Tiff, "image/tiff", "tiff"),
        (AttachmentType::Svg, "image/svg+xml", "svg"),
    ];

    for (att_type, expected_mime, expected_ext) in types {
        assert_eq!(att_type.mime_type(), expected_mime);
        assert_eq!(att_type.extension(), expected_ext);
    }
}

#[test]
fn test_all_text_types() {
    let types = vec![
        (AttachmentType::Text, "text/plain", "txt"),
        (AttachmentType::Html, "text/html", "html"),
        (AttachmentType::Xml, "application/xml", "xml"),
        (AttachmentType::Json, "application/json", "json"),
        (AttachmentType::Yaml, "application/yaml", "yaml"),
        (AttachmentType::Csv, "text/csv", "csv"),
        (AttachmentType::Tsv, "text/tab-separated-values", "tsv"),
    ];

    for (att_type, expected_mime, expected_ext) in types {
        assert_eq!(att_type.mime_type(), expected_mime);
        assert_eq!(att_type.extension(), expected_ext);
    }
}

#[test]
fn test_video_types() {
    let types = vec![
        (AttachmentType::Mp4, "video/mp4", "mp4"),
        (AttachmentType::Ogg, "video/ogg", "ogg"),
        (AttachmentType::Webm, "video/webm", "webm"),
    ];

    for (att_type, expected_mime, expected_ext) in types {
        assert_eq!(att_type.mime_type(), expected_mime);
        assert_eq!(att_type.extension(), expected_ext);
    }
}

#[test]
fn test_special_types() {
    assert_eq!(AttachmentType::UriList.mime_type(), "text/uri-list");
    assert_eq!(AttachmentType::UriList.extension(), "uri");

    assert_eq!(
        AttachmentType::ImageDiff.mime_type(),
        "application/vnd.allure.image.diff"
    );
    assert_eq!(AttachmentType::ImageDiff.extension(), "diff.png");
}
