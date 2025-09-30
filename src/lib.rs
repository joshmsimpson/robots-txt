//! # robotstxt
//!
//! A Rust library for parsing and querying robots.txt files.
//!
//! ## Features
//!
//! - Parse robots.txt from strings or URLs
//! - Check if paths are allowed for specific user-agents
//! - Extract sitemaps and comments
//! - Support for wildcards and path patterns
//!
//! ## Example
//!
//! ```rust
//! use robots_txt::RobotsTxt;
//!
//! # fn main() {
//! let content = r#"
//! User-agent: *
//! Disallow: /admin/
//! Allow: /public/
//! "#;
//!
//! let robots = RobotsTxt::parse(content);
//! assert!(robots.can_fetch("Googlebot", "/public/page.html"));
//! assert!(!robots.can_fetch("Googlebot", "/admin/panel"));
//! # }
//! ```

mod parser;

pub use parser::{RobotsTxt, RobotRule};