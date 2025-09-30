# robotstxt-rs

An intuitive Rust library for parsing and querying robots.txt files.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
robotstxt-rs = "0.1.1"
```

## Usage Examples

### Fetch from URL (async)

```rust
use robotstxt_rs::RobotsTxt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch robots.txt from a URL
    let robots = RobotsTxt::from_url("https://example.com/robots.txt").await?;

    // Check if a user-agent can fetch a path
    if robots.can_fetch("Googlebot", "/admin/secret") {
        println!("Allowed to crawl!");
    } else {
        println!("Blocked by robots.txt");
    }

    Ok(())
}
```

### Parse from string

```rust
use robotstxt_rs::RobotsTxt;

fn main() {
    let content = r#"
User-agent: *
Disallow: /admin/
Allow: /public/

User-agent: Googlebot
Disallow: /private/

Sitemap: https://example.com/sitemap.xml
# This is a comment
    "#;

    let robots = RobotsTxt::parse(content);

    // Check access for different user-agents
    println!("Can Mozilla access /public/? {}", robots.can_fetch("Mozilla", "/public/page.html"));
    println!("Can Mozilla access /admin/? {}", robots.can_fetch("Mozilla", "/admin/panel"));
    println!("Can Googlebot access /private/? {}", robots.can_fetch("Googlebot", "/private/data"));

    // Get sitemaps
    for sitemap in robots.get_sitemaps() {
        println!("Sitemap: {}", sitemap);
    }

    // Get comments
    for comment in robots.get_comments() {
        println!("Comment: {}", comment);
    }

    // Get all rules
    for (user_agent, rule) in robots.get_rules() {
        println!("User-agent: {}", user_agent);
        println!("  Allowed: {:?}", rule.allowed);
        println!("  Disallowed: {:?}", rule.disallowed);
    }

    // Get rules for a specific user-agent
    if let Some(rule) = robots.get_rule("Googlebot") {
        println!("Googlebot allowed paths: {:?}", rule.allowed);
        println!("Googlebot disallowed paths: {:?}", rule.disallowed);
    }

    // Get domain (if parsed from URL)
    if let Some(domain) = robots.get_domain() {
        println!("Domain: {}", domain);
    }
}
```

### Parse with custom domain

```rust
use robotstxt_rs::RobotsTxt;

fn main() {
    let content = r#"
User-agent: *
Disallow: /admin/
    "#;

    // Parse with a specific domain
    let robots = RobotsTxt::parse_with_domain(content, Some("example.com".to_string()));

    if let Some(domain) = robots.get_domain() {
        println!("Domain: {}", domain);
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2025
