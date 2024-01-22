use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    // We effectively create a SOA record from the beneath fields.
    pub ttl: Option<u32>,
    pub primary_nameserver: String,
    pub email: String,
    pub serial: u32,
    // Defaults to 86_400.
    pub refresh: Option<u32>,
    // Defaults to 7_200.
    pub retry: Option<u32>,
    // Defaults to 3_600_000.
    pub expire: Option<u32>,
    // Defaults to 172_800.
    pub minimum: Option<u32>,
    pub domain_name: String,

    // We present this as "records" externally,
    // but we actually handle it as a Vec of Subdomains
    // in order to preserve and read the mapping's key.
    pub records: Vec<Record>,
}

#[derive(Deserialize)]
pub struct Record {
    // name is our only mandatory key.
    // The other remain optional, and are validated after deserialization.
    pub name: String,
    // Juuuust in case.
    pub ttl: Option<u32>,

    pub a: Option<Vec<ARecord>>,
    pub aaaa: Option<Vec<AAAARecord>>,
    pub caa: Option<Vec<CAARecord>>,
    pub cname: Option<Vec<CNAMERecord>>,
    pub mx: Option<Vec<MXRecord>>,
    pub ns: Option<Vec<NSRecord>>,
    pub ptr: Option<Vec<PTRRecord>>,
    pub srv: Option<Vec<SRVRecord>>,
    pub txt: Option<Vec<TXTRecord>>,
}

#[derive(Deserialize)]
pub struct ARecord(pub String);

#[derive(Deserialize)]
pub struct AAAARecord(pub String);

#[derive(Deserialize)]
pub struct CAARecord {
    pub flags: u32,
    pub tag: String,
    pub ca_domain_name: String,
}

#[derive(Deserialize)]
pub struct CNAMERecord(pub String);

#[derive(Deserialize)]
pub struct MXRecord {
    pub mail_server: String,
    pub priority: u16,
}

#[derive(Deserialize)]
pub struct NSRecord(pub String);

#[derive(Deserialize)]
pub struct PTRRecord(pub String);

#[derive(Deserialize)]
pub struct SRVRecord {
    pub service: String,
    pub protocol: String,
    pub priority: u16,
    pub weight: u16,
    pub port: u16,
    pub target: String,
}

#[derive(Deserialize)]
pub struct TXTRecord(pub String);
