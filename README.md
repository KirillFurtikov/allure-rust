# allure-rust

> ⚠️ **Note**: This is an **unofficial** community-driven implementation of Allure reporting for Rust. It is not affiliated with or endorsed by the official Allure Framework project.

> 🚧 **Work in Progress**: This is an early version (WIP) and may contain bugs. Use with caution.

A Rust library for generating [Allure](https://docs.qameta.io/allure/) test reports with rich test execution details, steps, attachments, and more.

**Requirements:** [Allure CLI](https://allurereport.org/docs/gettingstarted-view-report/) must be installed to generate and view reports.


## Table of Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Quick Start](#quick-start)
- [Features](#features)
  - [Test Annotations](#test-annotations)
  - [Test Suites](#test-suites)
  - [Steps](#steps)
    - [Function Steps](#function-steps)
    - [Inline Steps](#inline-steps)
  - [Attachments](#attachments)
  - [Test Statuses](#test-statuses)
- [Examples](#examples)
- [Generating Reports](#generating-reports)
- [License](#license)

## Installation

Add `allure-rust` to your `Cargo.toml`:

```toml
[dev-dependencies]
allure-rust = "0.0.1"
```

## Configuration

By default, test results are written to the `allure-results` directory. You can customize this location using the `ALLURE_RESULTS_DIR` environment variable:

```bash
ALLURE_RESULTS_DIR=custom-results cargo test
```

## Quick Start

```rust
use allure_rust::{allure_test, allure_step};

#[allure_test("My first test")]
#[test]
fn test_example() {
    allure_step!("Step 1: Setup", {
        let value = 42;
        assert_eq!(value, 42);
    });

    allure_step!("Step 2: Verify", {
        assert!(true);
    });
}
```

## Features

### Test Annotations

Use `#[allure_test]` to mark your tests and provide custom titles:

```rust
use allure_rust::allure_test;

#[allure_test("Test with custom title")]
#[test]
fn test_custom_title() {
    assert_eq!(2 + 2, 4);
}

// Without custom title - uses function name
#[allure_test]
#[test]
fn test_default_title() {
    assert_eq!(1 + 1, 2);
}
```

### Test Suites

Group related tests using the `#[allure_suite]` attribute:

```rust
use allure_rust::{allure_suite, allure_test};

#[allure_suite("Calculator Tests")]
mod calculator {
    use super::*;

    #[allure_test("Addition test")]
    #[test]
    fn test_add() {
        assert_eq!(2 + 2, 4);
    }

    #[allure_test("Subtraction test")]
    #[test]
    fn test_subtract() {
        assert_eq!(5 - 3, 2);
    }
}
```

Tests without an explicit suite are automatically grouped by their module path.

### Steps

#### Function Steps

Use `#[step]` to mark functions as test steps. Parameters are automatically captured:

```rust
use allure_rust::{allure_test, step};

#[step("Calculate sum")]
fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}

#[step("Verify result")]
fn verify_result(actual: i32, expected: i32) {
    assert_eq!(actual, expected);
}

#[allure_test("Test with function steps")]
#[test]
fn test_with_steps() {
    let result = calculate_sum(2, 3);
    verify_result(result, 5);
}
```

#### Inline Steps

Use `allure_step!` macro to create steps inline:

```rust
use allure_rust::{allure_step, allure_test};

#[allure_test("Test with inline steps")]
#[test]
fn test_inline_steps() {
    allure_step!("Prepare data", {
        let data = vec![1, 2, 3, 4, 5];
        assert!(!data.is_empty());
    });

    allure_step!("Process data", {
        let sum: i32 = vec![1, 2, 3, 4, 5].iter().sum();
        assert_eq!(sum, 15);
    });
}
```

**Nested steps:**

```rust
#[allure_test("Test with nested steps")]
#[test]
fn test_nested() {
    allure_step!("Outer step", {
        let x = 10;
        
        allure_step!("Inner step 1", {
            assert!(x > 0);
        });
        
        allure_step!("Inner step 2", {
            assert_eq!(x, 10);
        });
    });
}
```

### Attachments

Attach various types of data to your tests:

```rust
use allure_rust::{add_attachment, allure_test, json};

#[allure_test("Test with attachments")]
#[test]
fn test_attachments() {
    // Text attachment
    add_attachment("log", "Test execution log");
    
    // JSON attachment
    add_attachment("data", json!({
        "status": "success",
        "count": 42
    }));
    
    // HTML attachment
    add_attachment("report", "<h1>Test Report</h1>");
}
```

**Supported attachment types:**

- **Text formats:** Text, HTML, XML, JSON, YAML, CSV, TSV, URI List
- **Images:** PNG, JPEG, GIF, BMP, TIFF, SVG, Image Diff
- **Video:** MP4, Ogg, Webm

**Explicit type specification:**

```rust
use allure_rust::{add_attachment_with_type, AttachmentType};

#[test]
fn test_explicit_type() {
    let image_data = vec![0u8; 100]; // Your image bytes
    add_attachment_with_type("screenshot", image_data, AttachmentType::Png);
}
```

### Test Statuses

Allure automatically captures different test outcomes:

```rust
use allure_rust::allure_test;

#[allure_test("Passed test")]
#[test]
fn test_passed() {
    assert_eq!(2 + 2, 4);
}

#[allure_test("Failed test")]
#[test]
fn test_failed() {
    assert_eq!(2 + 2, 5, "Math is broken!");
}

#[allure_test("Test with panic")]
#[test]
fn test_panic() {
    panic!("Something went wrong!");
}

#[allure_test("Skipped test")]
#[test]
#[ignore]
fn test_skipped() {
    assert_eq!(1, 1);
}
```

## Examples

### Complete Test Example

```rust
use allure_rust::{add_attachment, allure_step, allure_suite, allure_test, json, step};

#[allure_suite("User Management")]
mod user_tests {
    use super::*;

    #[step("Create user")]
    fn create_user(name: &str, age: i32) -> User {
        User {
            name: name.to_string(),
            age,
        }
    }

    #[step("Validate user")]
    fn validate_user(user: &User) {
        assert!(!user.name.is_empty());
        assert!(user.age > 0);
    }

    #[allure_test("Create and validate user")]
    #[test]
    fn test_user_creation() {
        allure_step!("Setup test data", {
            let name = "John Doe";
            let age = 30;
            
            add_attachment("test_data", json!({
                "name": name,
                "age": age
            }));
        });

        allure_step!("Create user", {
            let user = create_user("John Doe", 30);
            validate_user(&user);
        });

        allure_step!("Verify user properties", {
            let user = User {
                name: "John Doe".to_string(),
                age: 30,
            };
            assert_eq!(user.name, "John Doe");
            assert_eq!(user.age, 30);
        });
    }
}

struct User {
    name: String,
    age: i32,
}
```

### Testing with Assertions in Steps

```rust
use allure_rust::{allure_step, allure_test};

#[allure_test("Test with assertion steps")]
#[test]
fn test_assertions() {
    let numbers = vec![1, 2, 3, 4, 5];

    allure_step!("Verify vector is not empty", {
        assert!(!numbers.is_empty());
    });

    allure_step!("Verify vector length", {
        assert_eq!(numbers.len(), 5);
    });

    allure_step!("Calculate sum", {
        let sum: i32 = numbers.iter().sum();
        assert_eq!(sum, 15);
    });
}
```

## Generating Reports

After running your tests, generate the Allure report:

```bash
# Run tests
cargo test

# Generate report
allure generate --clean allure-results -o allure-report

# Open report in browser
allure open allure-report
```

### Crate Organization

- **`allure-rust`** - The main crate users add to their dependencies. It re-exports all functionality from `allure-rust-core`.
- **`allure-rust-core`** - Contains the core implementation of the library.
- **`allure-rust-macros`** - Provides procedural macros (`#[allure_test]`, `#[step]`, `#[allure_suite]`).
- **`test-suite`** - Comprehensive test suite with 52 tests and 80.92% code coverage.
- **`example`** - Example tests demonstrating library features.


## License

MIT
