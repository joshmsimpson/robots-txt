use robotstxt_rs::RobotsTxt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let robots = RobotsTxt::from_url("https://www.rust-lang.org/robots.txt").await?;
    
    if let Some(domain) = robots.get_domain() {
        println!("Domain: {}", domain);
    }
    
    println!("Can fetch /: {}", robots.can_fetch("MyBot", "/"));
    println!("Sitemaps found: {}", robots.get_sitemaps().len());
    
    Ok(())
}