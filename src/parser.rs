use std::collections::HashMap;

#[cfg(feature = "async")]
use reqwest;

/// A rule for a specific user-agent containing allowed and disallowed paths.
#[derive(Debug, Clone)]
pub struct RobotRule {
    /// The user-agent this rule applies to (e.g., "Googlebot", "*")
    pub user_agent: String,
    /// List of paths explicitly allowed for this user-agent
    pub allowed: Vec<String>,
    /// List of paths disallowed for this user-agent
    pub disallowed: Vec<String>,
}

/// The main structure representing a parsed robots.txt file.
#[derive(Debug)]
pub struct RobotsTxt {
    domain: Option<String>,
    rules: HashMap<String, RobotRule>,
    sitemaps: Vec<String>,
    comments: Vec<String>,
}

#[cfg(feature = "async")]

impl RobotsTxt {
    /// Parse a robots.txt file from a string.
    ///
    /// # Arguments
    ///
    /// * `content` - The robots.txt file content as a string
    ///
    /// # Example
    ///
    /// ```
    /// use robotstxt_rs::RobotsTxt;
    ///
    /// let content = "User-agent: *\nDisallow: /admin/";
    /// let robots = RobotsTxt::parse(content);
    /// ```
    pub fn parse(content: &str) -> Self {
        Self::parse_with_domain(content, None)
    }

    /// Parse a robots.txt file from a string with a specified domain.
    ///
    /// # Arguments
    ///
    /// * `content` - The robots.txt file content as a string
    /// * `domain` - Optional domain name to associate with this robots.txt
    ///
    /// # Example
    ///
    /// ```
    /// use robotstxt_rs::RobotsTxt;
    ///
    /// let content = "User-agent: *\nDisallow: /admin/";
    /// let robots = RobotsTxt::parse_with_domain(content, Some("example.com".to_string()));
    /// ```
    pub fn parse_with_domain(content: &str, domain: Option<String>) -> Self {
        let mut rules: HashMap<String, RobotRule> = HashMap::new();
        let mut sitemaps = Vec::new();
        let mut comments = Vec::new();

        let mut current_agents: Vec<String> = Vec::new();

        for line in content.lines() {
            let line = line.trim();

            // Handle comments
            if line.starts_with('#') {
                comments.push(line[1..].trim().to_string());
                continue;
            }

            // Skip empty lines
            if line.is_empty() {
                continue;
            }

            // Split on first colon
            if let Some((directive, value)) = line.split_once(':') {
                let directive = directive.trim().to_lowercase();
                let value = value.trim().to_string();

                match directive.as_str() {
                    "user-agent" => {
                        // Start new user-agent group
                        let agent = value.to_lowercase();
                        if !rules.contains_key(&agent) {
                            rules.insert(
                                agent.clone(),
                                RobotRule {
                                    user_agent: agent.clone(),
                                    allowed: Vec::new(),
                                    disallowed: Vec::new(),
                                },
                            );
                        }
                        current_agents.push(agent);
                    }
                    "allow" => {
                        // Add to all current agents
                        for agent in &current_agents {
                            if let Some(rule) = rules.get_mut(agent) {
                                rule.allowed.push(value.clone());
                            }
                        }
                    }
                    "disallow" => {
                        // Add to all current agents
                        for agent in &current_agents {
                            if let Some(rule) = rules.get_mut(agent) {
                                rule.disallowed.push(value.clone());
                            }
                        }
                    }
                    "sitemap" => {
                        sitemaps.push(value);
                        // Sitemap is global, reset current agents
                        current_agents.clear();
                    }
                    _ => {
                        // Unknown directive, could log or ignore
                    }
                }
            }
        }

        RobotsTxt {
            domain,
            rules,
            sitemaps,
            comments,
        }
    }

    /// Fetch and parse a robots.txt file from a URL (requires async feature).
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to the robots.txt file
    ///
    /// # Example
    ///
    /// ```no_run
    /// use robotstxt_rs::RobotsTxt;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let robots = RobotsTxt::from_url("https://example.com/robots.txt").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn from_url(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let content = client.get(url).send().await?.text().await?;

        // Extract domain from URL
        let domain = extract_domain(url);

        Ok(Self::parse_with_domain(&content, Some(domain)))
    }

    /// Check if a user-agent is allowed to fetch a specific path.
    ///
    /// # Arguments
    ///
    /// * `user_agent` - The user-agent string (e.g., "Googlebot")
    /// * `path` - The path to check (e.g., "/admin/panel")
    ///
    /// # Returns
    ///
    /// Returns `true` if the user-agent is allowed to fetch the path, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use robotstxt_rs::RobotsTxt;
    ///
    /// let content = "User-agent: *\nDisallow: /admin/";
    /// let robots = RobotsTxt::parse(content);
    /// assert!(!robots.can_fetch("Googlebot", "/admin/panel"));
    /// assert!(robots.can_fetch("Googlebot", "/public/page"));
    /// ```
    pub fn can_fetch(&self, user_agent: &str, path: &str) -> bool {
        let user_agent = user_agent.to_lowercase();

        // Try exact match first
        let rule = if let Some(rule) = self.rules.get(&user_agent) {
            rule
        } else if let Some(rule) = self.rules.get("*") {
            // Fall back to wildcard
            rule
        } else {
            // No rules = allowed
            return true;
        };

        // Check disallowed paths first (more restrictive)
        for disallowed in &rule.disallowed {
            if disallowed.is_empty() {
                continue;
            }
            if path_matches(path, disallowed) {
                // Check if there's a more specific allow rule
                for allowed in &rule.allowed {
                    if path_matches(path, allowed) && allowed.len() > disallowed.len() {
                        return true;
                    }
                }
                return false;
            }
        }

        // If not explicitly disallowed, check allowed rules
        // Empty allowed list means everything is allowed
        if rule.allowed.is_empty() {
            return true;
        }

        for allowed in &rule.allowed {
            if path_matches(path, allowed) {
                return true;
            }
        }

        // If there are allow rules but no match, it's disallowed
        false
    }

    /// Get the domain associated with this robots.txt file.
    ///
    /// # Returns
    ///
    /// Returns `Some(&str)` if a domain was specified, `None` otherwise.
    pub fn get_domain(&self) -> Option<&str> {
        self.domain.as_deref()
    }

    /// Get all sitemap URLs from the robots.txt file.
    ///
    /// # Returns
    ///
    /// Returns a slice of sitemap URLs.
    pub fn get_sitemaps(&self) -> &[String] {
        &self.sitemaps
    }

    /// Get all comments from the robots.txt file.
    ///
    /// # Returns
    ///
    /// Returns a slice of comment strings (without the # prefix).
    pub fn get_comments(&self) -> &[String] {
        &self.comments
    }

    /// Get all rules for all user-agents.
    ///
    /// # Returns
    ///
    /// Returns a HashMap where keys are user-agent strings and values are RobotRule structs.
    pub fn get_rules(&self) -> &HashMap<String, RobotRule> {
        &self.rules
    }

    /// Get the rule for a specific user-agent.
    ///
    /// # Arguments
    ///
    /// * `user_agent` - The user-agent string to look up
    ///
    /// # Returns
    ///
    /// Returns `Some(&RobotRule)` if rules exist for this user-agent or the wildcard (*), `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use robotstxt_rs::RobotsTxt;
    ///
    /// let content = "User-agent: Googlebot\nDisallow: /private/";
    /// let robots = RobotsTxt::parse(content);
    /// if let Some(rule) = robots.get_rule("Googlebot") {
    ///     println!("Disallowed paths: {:?}", rule.disallowed);
    /// }
    /// ```
    pub fn get_rule(&self, user_agent: &str) -> Option<&RobotRule> {
        let user_agent = user_agent.to_lowercase();
        self.rules.get(&user_agent).or_else(|| self.rules.get("*"))
    }
}

fn extract_domain(url: &str) -> String {
    // Simple domain extraction - handles common cases
    let url = url.trim();

    // Remove protocol
    let url = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    // Take everything before the first slash
    let domain = url.split('/').next().unwrap_or(url);

    // Remove port if present
    let domain = domain.split(':').next().unwrap_or(domain);

    domain.to_string()
}

fn path_matches(path: &str, pattern: &str) -> bool {
    // Handle end-of-string anchor $
    if pattern.ends_with('$') {
        let pattern = &pattern[..pattern.len() - 1];
        if pattern.contains('*') {
            // Complex wildcard matching with end anchor
            return path == pattern.replace('*', "");
        } else {
            return path == pattern;
        }
    }

    // Simple prefix matching (most common case)
    if !pattern.contains('*') {
        return path.starts_with(pattern);
    }

    // Handle wildcards
    let parts: Vec<&str> = pattern.split('*').collect();
    let mut pos = 0;

    for (i, part) in parts.iter().enumerate() {
        if i == 0 {
            // First part must be at the start
            if !path[pos..].starts_with(part) {
                return false;
            }
            pos += part.len();
        } else if i == parts.len() - 1 {
            // Last part must be at the end (or anywhere if followed by *)
            if !path[pos..].contains(part) {
                return false;
            }
        } else {
            // Middle parts just need to exist
            if let Some(found) = path[pos..].find(part) {
                pos += found + part.len();
            } else {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
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

        assert!(robots.can_fetch("Mozilla", "/public/test.html"));
        assert!(!robots.can_fetch("Mozilla", "/admin/panel"));
        assert!(!robots.can_fetch("Googlebot", "/private/data"));
        assert_eq!(robots.get_sitemaps().len(), 1);
        assert_eq!(robots.get_comments().len(), 1);
    }

    #[test]
    fn test_path_matching() {
        assert!(path_matches("/admin/test", "/admin/"));
        assert!(path_matches("/admin/", "/admin/"));
        assert!(!path_matches("/public/", "/admin/"));
        assert!(path_matches("/file.html", "/*.html"));
        assert!(path_matches("/admin/file.php", "/admin/*.php"));
        assert!(path_matches("/test.html", "/test.html$"));
        assert!(!path_matches("/test.html/more", "/test.html$"));
    }

    #[test]
    fn test_domain_extraction() {
        assert_eq!(
            extract_domain("https://example.com/robots.txt"),
            "example.com"
        );
        assert_eq!(
            extract_domain("http://www.google.com/robots.txt"),
            "www.google.com"
        );
        assert_eq!(
            extract_domain("https://api.github.com:443/robots.txt"),
            "api.github.com"
        );
        assert_eq!(extract_domain("example.org"), "example.org");
    }
}
