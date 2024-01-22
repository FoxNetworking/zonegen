use crate::yaml_format::{Configuration, Record};

pub fn spit_out_bind(config: Configuration) -> String {
    // First, some BIND-specific configuration.
    let mut contents = format!("$ORIGIN {}.\n", config.domain_name);

    // Next, we need to create our SOA record for this zone.
    // We'll generate this from top-level configuration.
    let effective_ttl = config.ttl.unwrap_or(3600);
    let domain_soa = create_soa(&config, effective_ttl);
    contents += &domain_soa;

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
    let a_records = record.a.as_deref().unwrap_or(&[]);
    for a_record in a_records {
        contents += &format!("{} {} IN A {}\n", name, ttl, a_record.0);
    }

    let aaaa_records = record.aaaa.as_deref().unwrap_or(&[]);
    for aaaa_record in aaaa_records {
        contents += &format!("{} {} IN AAAA {}\n", name, ttl, aaaa_record.0);
    }

    let cname_records = record.cname.as_deref().unwrap_or(&[]);
    for cname_record in cname_records {
        contents += &format!("{} {} IN CNAME {}.\n", name, ttl, cname_record.0);
    }

    let caa_records = record.caa.as_deref().unwrap_or(&[]);
    for caa_record in caa_records {
        contents += &format!(
            "{} {} IN CAA {} {} {}\n",
            name, ttl, caa_record.flags, caa_record.tag, caa_record.ca_domain_name
        );
    }

    let mx_records = record.mx.as_deref().unwrap_or(&[]);
    for mx_record in mx_records {
        contents += &format!(
            "{} {} IN MX {} {}.\n",
            name, ttl, mx_record.priority, mx_record.mail_server
        );
    }

    let ns_records = record.ns.as_deref().unwrap_or(&[]);
    for ns_record in ns_records {
        contents += &format!("{} {} IN NS {}.\n", name, ttl, ns_record.0);
    }

    let ptr_records = record.ptr.as_deref().unwrap_or(&[]);
    for ptr_record in ptr_records {
        contents += &format!("{} {} IN PTR {}.\n", name, ttl, ptr_record.0);
    }

    let srv_records = record.srv.as_deref().unwrap_or(&[]);
    for srv_record in srv_records {
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

    let txt_records = record.txt.as_deref().unwrap_or(&[]);
    for txt_record in txt_records {
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

    // The following defaults are known-good defaults.
    let effective_refresh = config.refresh.unwrap_or(86_400);
    let effective_retry = config.retry.unwrap_or(7_200);
    let effective_expire = config.expire.unwrap_or(3_600_000);
    let effective_minimum = config.minimum.unwrap_or(172_800);

    format!(
        "@ {} IN SOA {}. {}. {} {} {} {} {}\n",
        global_ttl,
        primary_ns,
        effective_email,
        serial,
        effective_refresh,
        effective_retry,
        effective_expire,
        effective_minimum
    )
}
