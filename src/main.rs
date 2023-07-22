use clap::Parser;
use ovh_dns_updater::ovh_dns_updater::OvhDnsUpdater;
use ovh_dns_updater::public_ip_retriever::IpRetrieverFromFasterService;
use std::env;

#[tokio::main]
async fn main() {
    let cmd_args = Args::parse();

    if !cmd_args.no_ip4 {
        let ip_retriever = IpRetrieverFromFasterService::new();
        let ovh_dns_updater = create_ovh_dns_updater_from_env_var();
        manage_ip4_record_upgrade(&ip_retriever, &ovh_dns_updater, &cmd_args.record).await;
    }
    if cmd_args.upgrade_ip6 {
        let ip_retriever = IpRetrieverFromFasterService::new();
        let ovh_dns_updater = create_ovh_dns_updater_from_env_var();
        manage_ip6_record_upgrade(&ip_retriever, &ovh_dns_updater, &cmd_args.record).await;
    }
}

fn create_ovh_dns_updater_from_env_var() -> OvhDnsUpdater {
    dotenv::dotenv().ok();
    let endpoint = match env::var("OVH_ENDPOINT") {
        Ok(endpoint) => endpoint,
        Err(_) => {
            eprintln!("OVH_ENDPOINT env variable must be set");
            std::process::exit(1);
        }
    };
    let ovh_application_key = match env::var("OVH_APPLICATION_KEY") {
        Ok(ovh_application_key) => ovh_application_key,
        Err(_) => {
            eprintln!("OVH_APPLICATION_KEY env variable must be set");
            std::process::exit(1);
        }
    };
    let ovh_application_secret = match env::var("OVH_APPLICATION_SECRET") {
        Ok(ovh_application_secret) => ovh_application_secret,
        Err(_) => {
            eprintln!("OVH_APPLICATION_SECRET env variable must be set");
            std::process::exit(1);
        }
    };
    let ovh_consumer_key = match env::var("OVH_CONSUMER_KEY") {
        Ok(ovh_consumer_key) => ovh_consumer_key,
        Err(_) => {
            eprintln!("OVH_CONSUMER_KEY env variable must be set");
            std::process::exit(1);
        }
    };
    match OvhDnsUpdater::new(
        &endpoint,
        &ovh_application_key,
        &ovh_application_secret,
        &ovh_consumer_key,
    ) {
        Ok(ovh_dns_updater) => ovh_dns_updater,
        Err(e) => {
            eprintln!("Error when creating OVH DNS updater: {:?}", e);
            std::process::exit(2);
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short)]
    no_ip4: bool,
    #[arg(long, short = '6')]
    upgrade_ip6: bool,
    #[arg(long, short)]
    record: String,
}

async fn manage_ip4_record_upgrade(
    ip_retriever: &IpRetrieverFromFasterService,
    ovh_dns_updater: &OvhDnsUpdater,
    record_to_update: &str,
) {
    let actual_ip4_future = ip_retriever.get_ip4();
    let recorded_ip4_future = ovh_dns_updater.get_dns_ipv4(record_to_update);
    let actual_ip4 = match actual_ip4_future.await {
        Some(actual_ip4) => actual_ip4,
        None => {
            eprintln!("Cannot retrieve current IPv4");
            std::process::exit(3);
        }
    };
    let recorded_ip4 = match recorded_ip4_future.await {
        Ok(recorded_ip4) => recorded_ip4,
        Err(e) => {
            eprintln!("Cannot retrieve IPv4 record: {:?}", e);
            std::process::exit(4);
        }
    };
    println!("Actual IP4: {:?}", actual_ip4);
    println!("Recorded IP4: {:?}", recorded_ip4);
    if actual_ip4 != recorded_ip4 {
        println!("Updating IP4 record...");
        match ovh_dns_updater
            .update_dns_ipv4(record_to_update, actual_ip4)
            .await
        {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Cannot update IPv4 record: {:?}", e);
                std::process::exit(5);
            }
        };
        println!("IP4 record updated");
    } else {
        println!("IP4 record is up to date");
    }
}

async fn manage_ip6_record_upgrade(
    ip_retriever: &IpRetrieverFromFasterService,
    ovh_dns_updater: &OvhDnsUpdater,
    record_to_update: &str,
) {
    let actual_ip6_future = ip_retriever.get_ip6();
    let recorded_ip6_future = ovh_dns_updater.get_dns_ipv6(record_to_update);
    let actual_ip6 = match actual_ip6_future.await {
        Some(actual_ip6) => actual_ip6,
        None => {
            eprintln!("Cannot retrieve current IPv6");
            std::process::exit(3);
        }
    };
    let recorded_ip6 = match recorded_ip6_future.await {
        Ok(recorded_ip6) => recorded_ip6,
        Err(e) => {
            eprintln!("Cannot retrieve IPv6 record: {:?}", e);
            std::process::exit(4);
        }
    };
    println!("Actual IP6: {:?}", actual_ip6);
    println!("Recorded IP6: {:?}", recorded_ip6);
    if actual_ip6 != recorded_ip6 {
        println!("Updating IP6 record");
        match ovh_dns_updater
            .update_dns_ipv6(record_to_update, actual_ip6)
            .await
        {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Cannot update IPv6 record: {:?}", e);
                std::process::exit(5);
            }
        };
        println!("IP6 record updated");
    } else {
        println!("IP6 record is up to date");
    }
}
