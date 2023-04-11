pub enum Reset {
    Latest,
    Earliest,
    None,
}
impl ToString for Reset {
    fn to_string(&self) -> String {
        match self {
            Reset::Latest => "latest",
            Reset::Earliest => "earliest",
            Reset::None => "none",
        }
        .to_string()
    }
}

pub enum DnsLookup {
    UseAllDnsIps,
    ResolveCanonicalBootstrapServersOnly,
}
impl ToString for DnsLookup {
    fn to_string(&self) -> String {
        match self {
            DnsLookup::UseAllDnsIps => "use_all_dns_ips",
            DnsLookup::ResolveCanonicalBootstrapServersOnly => {
                "resolve_canonical_bootstrap_servers_only"
            }
        }
        .to_string()
    }
}

pub enum RecordingLevel {
    Trace,
    Debug,
    Info,
}
impl ToString for RecordingLevel {
    fn to_string(&self) -> String {
        match self {
            RecordingLevel::Trace => "TRACE",
            RecordingLevel::Debug => "DEBUG",
            RecordingLevel::Info => "INFO",
        }
        .to_string()
    }
}

pub enum SecurityProtocol {
    Plaintext,
    Ssl,
    SaslPlaintext,
    SaslSsl,
}
impl ToString for SecurityProtocol {
    fn to_string(&self) -> String {
        match self {
            SecurityProtocol::Plaintext => "PLAINTEXT",
            SecurityProtocol::Ssl => "SSL",
            SecurityProtocol::SaslPlaintext => "SASL_PLAINTEXT",
            SecurityProtocol::SaslSsl => "SASL_SSL",
        }
        .to_string()
    }
}
