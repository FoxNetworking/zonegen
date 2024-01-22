use serde::Deserialize;

#[derive(Deserialize)]
// We effectively synthesize a SOA record from the fields in Configuration.
pub struct Configuration {
    // Defaults to 3_600.
    #[serde(default = "cfg_default_ttl")]
    pub ttl: u32,
    pub primary_nameserver: String,
    pub email: String,
    pub serial: u32,
    // Defaults to 86_400.
    #[serde(default = "cfg_default_refresh")]
    pub refresh: u32,
    // Defaults to 7_200.
    #[serde(default = "cfg_default_retry")]
    pub retry: u32,
    // Defaults to 3_600_000.
    #[serde(default = "cfg_default_expire")]
    pub expire: u32,
    // Defaults to 172_800.
    #[serde(default = "cfg_default_minimum")]
    pub minimum: u32,
    pub domain_name: String,

    // We present this as "records" externally,
    // but we actually handle it as a Vec of Subdomains
    // in order to preserve and read the mapping's key.
    pub records: Vec<Record>,
}

fn cfg_default_ttl() -> u32 {
    3_600
}

fn cfg_default_refresh() -> u32 {
    86_400
}

fn cfg_default_retry() -> u32 {
    7_200
}

fn cfg_default_expire() -> u32 {
    3_600_000
}

fn cfg_default_minimum() -> u32 {
    172_800
}

#[derive(Deserialize)]
pub struct Record {
    // name is our only mandatory key.
    // The other remain optional, and are validated after deserialization.
    pub name: String,
    // Juuuust in case.
    pub ttl: Option<u32>,

    #[serde(default = "Vec::new")]
    pub a: Vec<ARecord>,
    #[serde(default = "Vec::new")]
    pub aaaa: Vec<AAAARecord>,
    #[serde(default = "Vec::new")]
    pub caa: Vec<CAARecord>,
    #[serde(default = "Vec::new")]
    pub cname: Vec<CNAMERecord>,
    #[serde(default = "Vec::new")]
    pub mx: Vec<MXRecord>,
    #[serde(default = "Vec::new")]
    pub ns: Vec<NSRecord>,
    #[serde(default = "Vec::new")]
    pub ptr: Vec<PTRRecord>,
    #[serde(default = "Vec::new")]
    pub srv: Vec<SRVRecord>,
    #[serde(default = "Vec::new")]
    pub txt: Vec<TXTRecord>,
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
