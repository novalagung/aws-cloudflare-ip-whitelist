mod cloudflare_whitelist;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("security group ID must be specified as command argument");
    }
    let security_group_id = args.get(1).unwrap();
    println!("security_group_id: {security_group_id:?}");

    let mut ports: Vec<i32> = Vec::new();
    if args.len() >= 3 {
        for arg in &args[2..] {
            let mut p: Vec<i32> = arg.split(",")
                .map(|d| d.trim().parse::<i32>().unwrap_or_default())
                .filter(|d| *d != 0)
                .collect();
            ports.append(&mut p);
        }
    }
    if ports.len() == 0 {
        ports.push(80) // set default to http port
    }
    println!("ports: {ports:?})");

    cloudflare_whitelist::run(security_group_id, &ports).await
}

#[tokio::test]
async fn test_main() -> Result<(), Box<dyn std::error::Error>> {
    let security_group_id: &str = "sg-0cff43a33f085df79";
    cloudflare_whitelist::run(security_group_id).await
}
