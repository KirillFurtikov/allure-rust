use allure_rust::{AttachmentType, add_attachment, add_attachment_with_type, allure_test, json};

#[allure_test("Text attachments")]
#[test]
fn test_text_attachments() {
    add_attachment("plain_text", "This is a simple text attachment");

    add_attachment(
        "formatted_text",
        format!("Formatted text with values: {} and {}", 42, "test"),
    );

    add_attachment_with_type(
        "explicit_text",
        "Text with explicit type",
        AttachmentType::Text,
    );
}

#[allure_test("JSON attachments")]
#[test]
fn test_json_attachments() {
    add_attachment(
        "simple_json",
        json!({
            "name": "Test",
            "value": 123,
            "active": true
        }),
    );

    add_attachment(
        "complex_json",
        json!({
            "users": [
                {"name": "Alice", "age": 30},
                {"name": "Bob", "age": 25}
            ],
            "metadata": {
                "version": "1.0",
                "timestamp": 1234567890
            }
        }),
    );
}

#[allure_test("HTML attachments")]
#[test]
fn test_html_attachments() {
    add_attachment_with_type(
        "simple_html",
        "<html><body><h1>Test Report</h1><p>Status: Passed</p></body></html>",
        AttachmentType::Html,
    );

    add_attachment_with_type(
        "styled_html",
        r#"<html>
<head><style>body { font-family: Arial; } h1 { color: green; }</style></head>
<body>
    <h1>Test Results</h1>
    <ul>
        <li>Test 1: Passed</li>
        <li>Test 2: Passed</li>
        <li>Test 3: Failed</li>
    </ul>
</body>
</html>"#,
        AttachmentType::Html,
    );
}

#[allure_test("XML attachments")]
#[test]
fn test_xml_attachments() {
    add_attachment_with_type(
        "config_xml",
        r#"<?xml version="1.0" encoding="UTF-8"?>
<configuration>
    <database>
        <host>localhost</host>
        <port>5432</port>
    </database>
    <logging>
        <level>INFO</level>
    </logging>
</configuration>"#,
        AttachmentType::Xml,
    );
}

#[allure_test("YAML attachments")]
#[test]
fn test_yaml_attachments() {
    add_attachment_with_type(
        "config_yaml",
        r#"database:
  host: localhost
  port: 5432
  name: testdb
logging:
  level: INFO
  format: json
features:
  - authentication
  - authorization
  - logging"#,
        AttachmentType::Yaml,
    );
}

#[allure_test("CSV attachments")]
#[test]
fn test_csv_attachments() {
    add_attachment_with_type(
        "users_csv",
        "Name,Age,Email,Active\nAlice,30,alice@example.com,true\nBob,25,bob@example.com,false\nCharlie,35,charlie@example.com,true",
        AttachmentType::Csv,
    );

    add_attachment_with_type(
        "results_csv",
        "Test,Status,Duration\nTest 1,Passed,1.2s\nTest 2,Failed,0.8s\nTest 3,Passed,2.1s",
        AttachmentType::Csv,
    );
}

#[allure_test("TSV attachments")]
#[test]
fn test_tsv_attachments() {
    add_attachment_with_type(
        "data_tsv",
        "Name\tAge\tScore\nAlice\t30\t95\nBob\t25\t87\nCharlie\t35\t92",
        AttachmentType::Tsv,
    );
}

#[allure_test("URI list attachments")]
#[test]
fn test_uri_list_attachments() {
    add_attachment_with_type(
        "related_links",
        "https://example.com/test1\nhttps://example.com/test2\nhttps://example.com/test3",
        AttachmentType::UriList,
    );
}

#[allure_test("Image attachments")]
#[test]
fn test_image_attachments() {
    let fake_png = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    add_attachment_with_type("screenshot_png", fake_png.clone(), AttachmentType::Png);

    add_attachment_with_type("photo_jpeg", fake_png.clone(), AttachmentType::Jpeg);

    add_attachment_with_type("icon_gif", fake_png.clone(), AttachmentType::Gif);

    add_attachment_with_type("bitmap_bmp", fake_png.clone(), AttachmentType::Bmp);

    add_attachment_with_type("image_tiff", fake_png, AttachmentType::Tiff);
}

#[allure_test("SVG attachments")]
#[test]
fn test_svg_attachments() {
    add_attachment_with_type(
        "chart_svg",
        r#"<svg width="200" height="200" xmlns="http://www.w3.org/2000/svg">
    <circle cx="100" cy="100" r="80" fill="blue" />
    <text x="100" y="105" text-anchor="middle" fill="white" font-size="20">Test</text>
</svg>"#,
        AttachmentType::Svg,
    );

    add_attachment_with_type(
        "graph_svg",
        r#"<svg width="300" height="200">
    <rect x="10" y="10" width="80" height="100" fill="green" />
    <rect x="110" y="50" width="80" height="60" fill="red" />
    <rect x="210" y="30" width="80" height="80" fill="blue" />
</svg>"#,
        AttachmentType::Svg,
    );
}

#[allure_test("Image diff attachment")]
#[test]
fn test_image_diff() {
    let fake_diff = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    add_attachment_with_type("visual_diff", fake_diff, AttachmentType::ImageDiff);
}

#[allure_test("Mixed attachment types")]
#[test]
fn test_mixed_attachments() {
    add_attachment("description", "Test with multiple attachment types");

    add_attachment(
        "test_data",
        json!({
            "test_id": "mixed_001",
            "timestamp": 1234567890
        }),
    );

    add_attachment_with_type(
        "report_html",
        "<html><body><h2>Summary</h2><p>All checks passed</p></body></html>",
        AttachmentType::Html,
    );

    add_attachment_with_type(
        "metrics_csv",
        "Metric,Value\nDuration,1.5s\nMemory,128MB",
        AttachmentType::Csv,
    );

    let screenshot = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    add_attachment_with_type("final_state", screenshot, AttachmentType::Png);
}

#[allure_test("All supported attachment types")]
#[test]
fn test_all_types() {
    add_attachment("text", "Plain text");
    add_attachment_with_type(
        "html",
        "<html><body>HTML</body></html>",
        AttachmentType::Html,
    );
    add_attachment_with_type("xml", "<?xml version=\"1.0\"?><root/>", AttachmentType::Xml);
    add_attachment("json", json!({"type": "json"}));
    add_attachment_with_type("yaml", "key: value", AttachmentType::Yaml);
    add_attachment_with_type("csv", "A,B\n1,2", AttachmentType::Csv);
    add_attachment_with_type("tsv", "A\tB\n1\t2", AttachmentType::Tsv);
    add_attachment_with_type("uri", "https://example.com", AttachmentType::UriList);

    let img = vec![0x89, 0x50, 0x4E, 0x47];
    add_attachment_with_type("png", img.clone(), AttachmentType::Png);
    add_attachment_with_type("jpeg", img.clone(), AttachmentType::Jpeg);
    add_attachment_with_type("gif", img.clone(), AttachmentType::Gif);
    add_attachment_with_type("bmp", img.clone(), AttachmentType::Bmp);
    add_attachment_with_type("tiff", img.clone(), AttachmentType::Tiff);
    add_attachment_with_type("svg", "<svg></svg>", AttachmentType::Svg);
    add_attachment_with_type("diff", img, AttachmentType::ImageDiff);
}
