use dns_lookup::lookup_addr;
use std::net::IpAddr;

fn main() {
    let ip: IpAddr = "8.8.8.8".parse().unwrap();
    match lookup_addr(&ip) {
        Ok(hostname) => println!("Hostname: {}", hostname),
        Err(e) => eprintln!("Failed to lookup hostname: {}", e),
    }
}
