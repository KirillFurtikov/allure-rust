//! # allure-rust
//!
//! A Rust library for generating Allure test reports with rich test execution details,
//! steps, attachments, and more.
//!
//! This is a facade crate that re-exports all functionality from `allure-rust-core`.
//!
//! ## Quick Start
//!
//! ```rust
//! use allure_rust::{allure_test, allure_step};
//!
//! #[allure_test("My first test")]
//! #[test]
//! fn test_example() {
//!     allure_step!("Step 1: Setup", {
//!         let value = 42;
//!         assert_eq!(value, 42);
//!     });
//!
//!     allure_step!("Step 2: Verify", {
//!         assert!(true);
//!     });
//! }
//! ```
//!
//! For more examples and documentation, see the [README](https://github.com/yourusername/allure-rust).

pub use allure_rust_core::*;
