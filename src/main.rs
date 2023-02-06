mod cloudflare_whitelist;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("security group ID must be specified as command argument");
    }

    let security_group_id = args.get(1).unwrap();
    cloudflare_whitelist::run(security_group_id).await
}

#[tokio::test]
async fn test_main() -> Result<(), Box<dyn std::error::Error>> {
    let security_group_id: &str = "sg-0cff43a33f085df79";
    cloudflare_whitelist::run(security_group_id).await
}
