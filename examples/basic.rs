use robotstxt_rs::RobotsTxt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Example 1: Fetch from URL (async) - Accessing all fields ===\n");

    // Fetch robots.txt from a URL
    let robots = RobotsTxt::from_url("https://www.docs.rs/robots.txt").await?;

    // Get and print domain
    if let Some(domain) = robots.get_domain() {
        println!("Domain: {}", domain);
    } else {
        println!("Domain: None");
    }

    // Get and print all sitemaps
    println!("\nSitemaps ({} found):", robots.get_sitemaps().len());
    for sitemap in robots.get_sitemaps() {
        println!("  - {}", sitemap);
    }

    // Get and print all comments
    println!("\nComments ({} found):", robots.get_comments().len());
    for comment in robots.get_comments() {
        println!("  - {}", comment);
    }

    // Get and print all rules with full RobotRule struct fields
    println!("\nRules for all user-agents:");
    for (user_agent_key, rule) in robots.get_rules() {
        println!("  User-agent key: {}", user_agent_key);
        println!("    RobotRule.user_agent: {}", rule.user_agent);
        println!("    RobotRule.allowed: {:?}", rule.allowed);
        println!("    RobotRule.disallowed: {:?}", rule.disallowed);
    }

    // Check if a user-agent can fetch a path
    println!("\nAccess checks:");
    if robots.can_fetch("Googlebot", "/") {
        println!("  Googlebot: Allowed to crawl /");
    } else {
        println!("  Googlebot: Blocked by robots.txt");
    }

    // Get specific rule and print all its fields
    if let Some(rule) = robots.get_rule("*") {
        println!("\nSpecific rule for wildcard (*):");
        println!("  user_agent field: {}", rule.user_agent);
        println!("  allowed field: {:?}", rule.allowed);
        println!("  disallowed field: {:?}", rule.disallowed);
    }

    println!("\n=== Example 2: Parse from string ===\n");

    let content = r#"
User-agent: *
Disallow: /admin/
Allow: /public/

User-agent: Googlebot
Disallow: /private/

Sitemap: https://example.com/sitemap.xml
Sitemap: https://example.com/sitemap2.xml
# This is a comment
# Another comment
    "#;

    let robots = RobotsTxt::parse(content);

    // Check access for different user-agents
    println!(
        "Can Mozilla access /public/? {}",
        robots.can_fetch("Mozilla", "/public/page.html")
    );
    println!(
        "Can Mozilla access /admin/? {}",
        robots.can_fetch("Mozilla", "/admin/panel")
    );
    println!(
        "Can Googlebot access /private/? {}",
        robots.can_fetch("Googlebot", "/private/data")
    );

    println!("\n=== Example 3: Get sitemaps ===\n");

    // Get sitemaps
    for sitemap in robots.get_sitemaps() {
        println!("Sitemap: {}", sitemap);
    }

    println!("\n=== Example 4: Get comments ===\n");

    // Get comments
    for comment in robots.get_comments() {
        println!("Comment: {}", comment);
    }

    println!("\n=== Example 5: Get all rules ===\n");

    // Get all rules
    for (user_agent, rule) in robots.get_rules() {
        println!("User-agent: {}", user_agent);
        println!("  Allowed: {:?}", rule.allowed);
        println!("  Disallowed: {:?}", rule.disallowed);
    }

    println!("\n=== Example 6: Get rules for specific user-agent ===\n");

    // Get rules for a specific user-agent
    if let Some(rule) = robots.get_rule("Googlebot") {
        println!("Googlebot user_agent: {}", rule.user_agent);
        println!("Googlebot allowed paths: {:?}", rule.allowed);
        println!("Googlebot disallowed paths: {:?}", rule.disallowed);
    }

    println!("\n=== Example 7: Parse with custom domain ===\n");

    let content2 = r#"
User-agent: *
Disallow: /admin/
    "#;

    // Parse with a specific domain
    let robots = RobotsTxt::parse_with_domain(content2, Some("example.com".to_string()));

    if let Some(domain) = robots.get_domain() {
        println!("Custom domain: {}", domain);
    }

    println!("\n=== All examples completed successfully! ===");

    Ok(())
}
