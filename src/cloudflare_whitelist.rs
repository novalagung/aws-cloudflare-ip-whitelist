use aws_sdk_ec2::{Client};
use aws_sdk_ec2::model::{Filter, Ipv6Range, IpRange};

pub async fn run(security_group_id: &str, ports: &Vec<i32>) -> Result<(), Box<dyn std::error::Error>> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    // ============== get current security groups ==============
    
    println!("getting current security group rules");

    let mut current_ingress_ids: Vec<String> = Vec::new();
    
    let security_group_filter = Filter::builder()
        .set_name(Some("group-id".to_string()))
        .set_values(Some(vec![security_group_id.to_string()]))
        .build();

    let resp = client.describe_security_group_rules()
        .set_filters(Some(vec![security_group_filter]))
        .send()
        .await?;

    for rule in resp.security_group_rules().unwrap_or_default() {
        if rule.is_egress().unwrap_or_default() {
            continue
        }
        let group_id = rule.security_group_rule_id().unwrap_or_default().to_string();
        if group_id.len() == 0 {
            continue
        }
        current_ingress_ids.push(group_id);
    }

    println!("   => done");

    // ============== remove ingress rules ==============

    if current_ingress_ids.len() > 0 {
        println!("remove current ingress rules (ingress_ids: {current_ingress_ids:?})");
        
        let resp = client.revoke_security_group_ingress()
            .set_group_id(Some(security_group_id.to_string()))
            .set_security_group_rule_ids(Some(current_ingress_ids))
            .send()
            .await?;
    
        println!("   => done ({})", resp.r#return().unwrap_or_default());
    }
    
    // ============== get cloudflare whitelist ips ==============

    println!("get cloudflare whitelist ips");
    
    let mut ips: Vec<String> = Vec::new();

    let cloudflare_ip_list = [
        "https://www.cloudflare.com/ips-v4",
        "https://www.cloudflare.com/ips-v6",
    ];

    for ip_list in cloudflare_ip_list {
        let resp_ip = reqwest::get(ip_list)
            .await?.text().await?;
        
        resp_ip.split("\n")
            .filter(|d| d.len() > 0)
            .for_each(|d| ips.push(d.to_string()));
    
        println!("   => done ({})", ip_list);
    }

    // ============== add ingress rules ==============

    println!("add new security group rules");

    for ip in ips {
        for port in ports {
            let mut ip_permission = aws_sdk_ec2::model::IpPermission::builder()
                .set_ip_protocol(Some("tcp".to_string()))
                .set_from_port(Some(*port))
                .set_to_port(Some(*port));
    
            if ip.contains("::") {
                ip_permission = ip_permission.set_ipv6_ranges(Some(vec![
                    Ipv6Range::builder().set_cidr_ipv6(Some(ip.to_string())).build()
                ]))
            } else {
                ip_permission = ip_permission.set_ip_ranges(Some(vec![
                    IpRange::builder().set_cidr_ip(Some(ip.to_string())).build()
                ]))
            }
            
            let resp = client.authorize_security_group_ingress()
                .set_group_id(Some(security_group_id.to_string()))
                .set_ip_permissions(Some(vec![ip_permission.build()]))
                .send()
                .await?;
        
            println!("   => done ({} {} {})", ip, *port, resp.r#return().unwrap_or_default());
        }
    }

    Ok(())
}
