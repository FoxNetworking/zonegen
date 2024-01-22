use crate::yaml_format::{Configuration, Record};

pub fn spit_out_bind(config: Configuration) -> String {
    // First, some BIND-specific configuration.
    let mut contents = format!("$ORIGIN {}.\n", config.domain_name);

    // Next, we need to create our SOA record for this zone.
    // We'll generate this from top-level configuration.
    contents += &domain_soa;
    let global_ttl = config.ttl;
    let domain_soa = create_soa(&config, global_ttl);

    // Next, we'll synthesize all record types.
    for record in config.records.iter() {
        contents += &spit_out_record(record, effective_ttl);
    }

    contents
}

pub fn spit_out_record(record: &Record, global_ttl: u32) -> String {
    let mut contents = "".to_string();
    let name = &record.name;
    let ttl = record.ttl.unwrap_or(global_ttl);

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

    contents
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
