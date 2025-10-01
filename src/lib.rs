//! # robotstxt-rs
//!
//! An intuitive Rust library for acquiring, parsing and querying robots.txt files.
//!
//! ## Features
//!
//! - Parse robots.txt from strings or URLs
//! - Check if paths are allowed for specific user-agents
//! - Extract sitemaps and comments
//! - Support for wildcards and path patterns
//! - Async URL fetching with reqwest
//!
//! ## Examples
//!
//! ### Basic parsing and querying
//!
//! ```rust
//! use robotstxt_rs::RobotsTxt;
//!
//! # fn main() {
//! let content = r#"
//! User-agent: *
//! Disallow: /admin/
//! Allow: /public/
//!
//! User-agent: Googlebot
//! Disallow: /private/
//!
//! Sitemap: https://example.com/sitemap.xml
//! # This is a comment
//! "#;
//!
//! let robots = RobotsTxt::parse(content);
//!
//! // Check if paths are allowed
//! assert!(robots.can_fetch("Mozilla", "/public/page.html"));
//! assert!(!robots.can_fetch("Mozilla", "/admin/panel"));
//! assert!(!robots.can_fetch("Googlebot", "/private/data"));
//!
//! // Access sitemaps
//! for sitemap in robots.get_sitemaps() {
//!     println!("Sitemap: {}", sitemap);
//! }
//!
//! // Access comments
//! for comment in robots.get_comments() {
//!     println!("Comment: {}", comment);
//! }
//!
//! // Get all rules
//! for (user_agent, rule) in robots.get_rules() {
//!     println!("User-agent: {}", user_agent);
//!     println!("  Allowed: {:?}", rule.allowed);
//!     println!("  Disallowed: {:?}", rule.disallowed);
//! }
//!
//! // Get specific rule
//! if let Some(rule) = robots.get_rule("Googlebot") {
//!     println!("Googlebot disallowed: {:?}", rule.disallowed);
//! }
//! # }
//! ```
//!
//! ### Fetch from URL (async)
//!
//! ```no_run
//! use robotstxt_rs::RobotsTxt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let robots = RobotsTxt::from_url("https://example.com/robots.txt").await?;
//!
//!     if let Some(domain) = robots.get_domain() {
//!         println!("Domain: {}", domain);
//!     }
//!
//!     if robots.can_fetch("MyBot", "/") {
//!         println!("Allowed to crawl!");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Parse with custom domain
//!
//! ```rust
//! use robotstxt_rs::RobotsTxt;
//!
//! # fn main() {
//! let content = "User-agent: *\nDisallow: /admin/";
//! let robots = RobotsTxt::parse_with_domain(content, Some("example.com".to_string()));
//!
//! if let Some(domain) = robots.get_domain() {
//!     println!("Domain: {}", domain);
//! }
//! # }
//! ```

mod parser;

pub use parser::{RobotsTxt, RobotRule};