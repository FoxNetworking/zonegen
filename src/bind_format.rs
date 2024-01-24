use crate::{
    ip_address::{format_v4, format_v6},
    yaml_format::{Configuration, Record},
};
use anyhow::Result;
use std::{
    fmt::Write,
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

pub fn config_to_zone(config: Configuration) -> Result<String> {
    // First, some BIND-specific configuration.
    let mut contents = format!("$ORIGIN {}.\n", config.domain_name);

    // Next, we need to create our SOA record for this zone.
    // We'll generate this from top-level configuration.
    let global_ttl = config.ttl;
    let domain_soa = create_soa(&config, global_ttl);
    write!(contents, "{}", domain_soa)?;

    // Next, we'll synthesize all record types.
    for record in config.records.iter() {
        write!(
            contents,
            "{}",
            &spit_out_record(record, &config.domain_name, global_ttl)?
        )?;
    }

    Ok(contents)
}

fn determine_record_name(record: &Record, domain_name: &String) -> Result<String> {
    // We'll have to synthesize a base name from three sources:
    //  - `name`, as a subdomain
    //  - `ip4`, as a partial IPv4 segment
    //  - `ip6`, as a partial IPv6 segment
    if let Some(record_name) = &record.name {
        Ok(record_name.to_string())
    } else if let Some(ip4) = &record.ip4 {
        let parsed_v4 = Ipv4Addr::from_str(ip4)?;
        let name: String = format_v4(parsed_v4, domain_name)?;
        Ok(name.to_string())
    } else if let Some(ip6) = &record.ip6 {
        let parsed_v6 = Ipv6Addr::from_str(ip6)?;
        let name: String = format_v6(parsed_v6, domain_name)?;
        Ok(name.to_string())
    } else {
        panic!("No subdomain, IPv4 address, or IPv6 address was specified for a record!");
    }
}

pub fn spit_out_record(record: &Record, domain_name: &String, global_ttl: u32) -> Result<String> {
    let name = determine_record_name(record, domain_name)?;
    let ttl = record.ttl.unwrap_or(global_ttl);

    let mut contents = "".to_string();

    // First, handle A and AAAA records.
    for a_record in &record.a {
        contents += &format!("{} {} IN A {}\n", name, ttl, a_record.0);
    }

    for aaaa_record in &record.aaaa {
        contents += &format!("{} {} IN AAAA {}\n", name, ttl, aaaa_record.0);
    }

    for cname_record in &record.cname {
        contents += &format!("{} {} IN CNAME {}.\n", name, ttl, cname_record.0);
    }

    for caa_record in &record.caa {
        contents += &format!(
            "{} {} IN CAA {} {} {}\n",
            name, ttl, caa_record.flags, caa_record.tag, caa_record.ca_domain_name
        );
    }

    for mx_record in &record.mx {
        contents += &format!(
            "{} {} IN MX {} {}.\n",
            name, ttl, mx_record.priority, mx_record.mail_server
        );
    }

    for ns_record in &record.ns {
        contents += &format!("{} {} IN NS {}.\n", name, ttl, ns_record.0);
    }

    for ptr_record in &record.ptr {
        contents += &format!("{} {} IN PTR {}.\n", name, ttl, ptr_record.0);
    }

    for srv_record in &record.srv {
        contents += &format!(
            "{}.{}.{} {} IN SRV {} {} {} {}.\n",
            srv_record.service,
            srv_record.protocol,
            name,
            ttl,
            srv_record.priority,
            srv_record.weight,
            srv_record.port,
            srv_record.target
        );
    }

    for txt_record in &record.txt {
        let raw_record = &txt_record.0.replace('\n', "\\n").replace('\"', "\\\"");

        contents += &format!("{} {} IN TXT \"{}\"\n", name, ttl, raw_record);
    }

    Ok(contents)
}

fn create_soa(config: &Configuration, global_ttl: u32) -> String {
    let primary_ns = &config.primary_nameserver;
    let serial = config.serial;

    let Some((local_part, email_domain)) = config.email.split_once('@') else {
        panic!("Invalid email format! Ensure your email has a single commercial at symbol in it.");
    };
    // We'll need to replace all periods within the email's local part to be escaped.
    let fixed_local_part = local_part.replace('.', "\\.");
    let effective_email = format!("{}.{}", fixed_local_part, email_domain);

    format!(
        "@ {} IN SOA {}. {}. {} {} {} {} {}\n",
        global_ttl,
        primary_ns,
        effective_email,
        serial,
        config.refresh,
        config.retry,
        config.expire,
        config.minimum
    )
}
