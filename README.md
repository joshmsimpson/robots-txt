# robots-txt

An intuitive Rust library for parsing and querying robots.txt files.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
robotstxt = "0.1"
```

## Usage Example

```rust
use robotstxt::RobotsTxt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch from URL
    let robots = RobotsTxt::from_url("https://example.com/robots.txt").await?;

    // Check if allowed
    if robots.can_fetch("Googlebot", "/admin/secret") {
        println!("Allowed to crawl!");
    }

    Ok(())
}
```
