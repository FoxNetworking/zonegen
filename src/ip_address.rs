use anyhow::{Ok, Result};
use std::{
    fmt::Write,
    net::{Ipv4Addr, Ipv6Addr},
};

pub fn format_v4(address: Ipv4Addr, base_domain: &String) -> Result<String> {
    // First, let's ensure that our base domain actually is in-addr.arpa.
    if !base_domain.ends_with(".in-addr.arpa") {
        panic!("Base domain for IPv4 record does not end with in-addr!")
    }

    // Reverse our IPv4 address for in-addr.arpa.
    let v4_string = address.to_string();
    let reversed_string: String = v4_string
        .split('.')
        .rev()
        .map(|char| format!("{}.", char))
        .collect();

    // Lastly, remove our base domain (assumed to be 'in-addr.arpa'.)
    let mut output = format!("{}in-addr.arpa", reversed_string);
    // We assume input of '2.0.192.in-addr.arpa'.
    // In order to fully remove our base domain, we'll need to replace
    // it alongside a prefixed period.
    let period_prefixed_domain = format!(".{}", base_domain);
    output = output.replace(&period_prefixed_domain, "");
    Ok(output)
}

pub fn format_v6(address: Ipv6Addr, base_domain: &String) -> Result<String> {
    // First, let's ensure that our base domain actually is ip6.arpa.
    if !base_domain.ends_with(".ip6.arpa") {
        panic!("Base domain for IPv6 record does not end with ip6.arpa!")
    }

    // Expand an IPv6 address fully.
    // '2001:db8::1' => '20010db8000000000000000000000001'
    let mut contents = "".to_string();
    let octets = address.octets();
    for octet in octets {
        write!(contents, "{:02x}", octet)?;
    }

    // Next, iterate backwards and insert periods throughout.
    // i.e. '20010db8000000000000000000000001' =>
    // 1.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.8.b.d.0.1.0.0.2.
    let eventual_output: String = contents
        .chars()
        .rev()
        .map(|char| format!("{}.", char))
        .collect();

    // Lastly, remove our base domain (assumed to be 'x.x.x.x.ip6.arpa'.)
    let mut output = format!("{}ip6.arpa", eventual_output);
    // We assume input of '0.8.b.d.0.1.0.0.2.ip6.arpa'.
    // In order to fully remove our base domain, we'll need to replace
    // it alongside a prefixed period.
    let period_prefixed_domain = format!(".{}", base_domain);
    output = output.replace(&period_prefixed_domain, "");
    Ok(output)
}
