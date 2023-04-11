use std::time::Duration;

use super::types::{DnsLookup, RecordingLevel, SecurityProtocol};

pub trait Set {
    fn set(&mut self, key: &str, value: impl ToString);
}

pub trait KafkaConfigBuilder: Set + Sized {
    /// A list of host/port pairs to use for establishing the initial connection to the Kafka
    /// cluster. The client will make use of all servers irrespective of which servers are
    /// specified here for bootstrapping—this list only impacts the initial hosts used to discover
    /// the full set of servers. This list should be in the form `host1:port1,host2:port2,....`
    /// Since these servers are just used for the initial connection to discover the full cluster
    /// membership (which may change dynamically), this list need not contain the full set of
    /// servers (you may want more than one, though, in case a server is down).
    ///
    /// Default: ""
    fn bootstrap_servers(mut self, urls: &[impl AsRef<str>]) -> Self {
        self.set(
            "bootstrap.servers",
            urls.iter()
                .flat_map(|val| [val.as_ref(), ","])
                .collect::<String>(),
        );
        self
    }
    /// Controls how the client uses DNS lookups. If set to `use_all_dns_ips`, connect to each returned IP address in sequence until a successful connection is established. After a disconnection, the next IP is used. Once all IPs have been used once, the client resolves the IP(s) from the hostname again (both the JVM and the OS cache DNS name lookups, however). If set to `resolve_canonical_bootstrap_servers_only`, resolve each bootstrap address into a list of canonical names. After the bootstrap phase, this behaves the same as `use_all_dns_ips`.
    ///
    /// Default: use_all_dns_ips
    fn client_dns_lookup(mut self, lookup: DnsLookup) -> Self {
        self.set("client.dns.lookup", lookup);
        self
    }
    /// An id string to pass to the server when making requests. The purpose of this is to be able to track the source of requests beyond just ip/port by allowing a logical application name to be included in server-side request logging.
    ///
    /// Default: ""
    fn client_id(mut self, id: &str) -> Self {
        self.set("client.id", id);
        self
    }
    /// Close idle connections after the number of milliseconds specified by this config.
    ///
    /// Default: 300000 (5 minutes)
    fn connections_max_idle(mut self, idle: Duration) -> Self {
        self.set("connections.max.idle.ms", idle.as_millis());
        self
    }
    /// The period of time in milliseconds after which we force a refresh of metadata even if we haven’t seen any partition leadership changes to proactively discover any new brokers or partitions.
    ///
    /// Default: 300000 (5 minutes)
    fn metadata_max_age(mut self, age: Duration) -> Self {
        self.set("metadata.max.age.ms", age.as_millis());
        self
    }
    /// A list of classes to use as metrics reporters. Implementing the `org.apache.kafka.common.metrics.MetricsReporter` interface allows plugging in classes that will be notified of new metric creation. The JmxReporter is always included to register JMX statistics.
    ///
    /// Default: ""
    fn metric_reporters(mut self, vals: &[&impl AsRef<str>]) -> Self {
        self.set(
            "metric.reporters",
            vals.iter()
                .flat_map(|val| [val.as_ref(), ","])
                .collect::<String>(),
        );
        self
    }
    /// The number of samples maintained to compute metrics.
    ///
    /// Default: 2
    fn metrics_num_samples(mut self, val: usize) -> Self {
        self.set("metrics.num.samples", val);
        self
    }
    /// The highest recording level for metrics.
    ///
    /// Default: INFO
    fn metrics_recording_level(mut self, val: RecordingLevel) -> Self {
        self.set("metrics.recording.level", val);
        self
    }
    /// The window of time a metrics sample is computed over.
    ///
    /// Defualt: 30000 (30 seconds)
    fn metrics_sample_window(mut self, window: Duration) -> Self {
        self.set("metrics.sample.window.ms", window.as_millis());
        self
    }
    /// The size of the TCP receive buffer (SO_RCVBUF) to use when reading data. If the value is -1, the OS default will be used.
    ///
    /// Default: 65536 (64 kibibytes)
    fn receive_buffer_bytes(mut self, count: isize) -> Self {
        self.set("receive.buffer.bytes", count);
        self
    }
    /// The maximum amount of time in milliseconds to wait when reconnecting to a broker that has repeatedly failed to connect. If provided, the backoff per host will increase exponentially for each consecutive connection failure, up to this maximum. After calculating the backoff increase, 20% random jitter is added to avoid connection storms.
    ///
    /// Default: 1000 (1 seconds)
    fn reconnect_backoff_max(mut self, max: Duration) -> Self {
        self.set("reconnect.backoff.max.ms", max.as_millis());
        self
    }
    /// The base amount of time to wait before attempting to reconnect to a given host. This avoids repeatedly connecting to a host in a tight loop. This backoff applies to all connection attempts by the client to a broker.
    ///
    /// Default: 50
    fn reconnect_backoff(mut self, backoff: Duration) -> Self {
        self.set("reconnect.backoff.ms", backoff.as_millis());
        self
    }
    /// The configuration controls the maximum amount of time the client will wait for the response of a request. If the response is not received before the timeout elapses the client will resend the request if necessary or fail the request if retries are exhausted.
    ///
    /// Default: 30000 (30 seconds)
    fn request_timeout(mut self, timeout: Duration) -> Self {
        self.set("request.timeout.ms", timeout.as_millis());
        self
    }
    /// The amount of time to wait before attempting to retry a failed request. This avoids repeatedly sending requests in a tight loop under some failure scenarios.
    ///
    /// Default: 100
    fn retry_backoff(mut self, backoff: Duration) -> Self {
        self.set("retry.backoff.ms", backoff.as_millis());
        self
    }
    /// Protocol used to communicate with brokers. Valid values are: PLAINTEXT, SSL, SASL_PLAINTEXT, SASL_SSL.
    ///
    /// Default: PLAINTEXT
    fn security_protocol(mut self, protocol: SecurityProtocol) -> Self {
        self.set("security.protocol", protocol);
        self
    }
    /// A list of configurable creator classes each returning a provider implementing security algorithms. These classes should implement the `org.apache.kafka.common.security.auth.SecurityProviderCreator` interface.
    ///
    /// Default: null
    fn security_providers(mut self, vals: &[&impl AsRef<str>]) -> Self {
        self.set(
            "security.providers",
            vals.iter()
                .flat_map(|val| [val.as_ref(), ","])
                .collect::<String>(),
        );
        self
    }
    /// The size of the TCP send buffer (SO_SNDBUF) to use when sending data. If the value is -1, the OS default will be used.
    ///
    /// Default: 131072 (128 kibibytes)
    fn send_buffer_bytes(mut self, count: usize) -> Self {
        self.set("send.buffer.bytes", count);
        self
    }
    /// The maximum amount of time the client will wait for the socket connection to be established. The connection setup timeout will increase exponentially for each consecutive connection failure up to this maximum. To avoid connection storms, a randomization factor of 0.2 will be applied to the timeout resulting in a random range between 20% below and 20% above the computed value.
    ///
    /// Default: 30000 (30 seconds)
    fn socket_connection_setup_timeout_max(mut self, max: Duration) -> Self {
        self.set("socket.connection.setup.timeout.max.ms", max.as_millis());
        self
    }
    /// The amount of time the client will wait for the socket connection to be established. If the connection is not built before the timeout elapses, clients will close the socket channel.
    ///
    /// Default: 10000 (10 seconds)
    fn socket_connection_setup_timeout(mut self, val: Duration) -> Self {
        self.set("socket.connection.setup.timeout.ms", val.as_millis());
        self
    }
}

pub trait SslConfigBuilder: Set + Sized {
    /// The password of the private key in the key store file or the PEM key specified in `ssl.keystore.key`.
    ///
    /// Default: null
    fn ssl_key_password(mut self, password: &str) -> Self {
        self.set("ssl.key.password", password);
        self
    }
    /// Certificate chain in the format specified by ‘ssl.keystore.type’. Default SSL engine factory supports only PEM format with a list of X.509 certificates
    fn ssl_keystore_certificate_chain(mut self, chain: &str) -> Self {
        self.set("ssl.keystore.certificate.chain", chain);
        self
    }
    /// Private key in the format specified by ‘ssl.keystore.type’. Default SSL engine factory supports only PEM format with PKCS#8 keys. If the key is encrypted, key password must be specified using ‘ssl.key.password’
    ///
    /// Default: null
    fn ssl_keystore_key(mut self, key: &str) -> Self {
        self.set("ssl.keystore.key", key);
        self
    }
    // I'm sure there's more
}

pub trait SaslConfigBuilder: Set + Sized {
    // There are some for this
}

pub trait ApiTimeoutConfigBuilder: Set + Sized {
    fn default_api_timeout(mut self, timeout: Duration) -> Self {
        self.set("default.api.timeout.ms", timeout.as_micros());
        self
    }
}

pub trait RetriesConfigBuilder: Set + Sized {
    fn retries(mut self, count: usize) -> Self {
        self.set("retries", count);
        self
    }
}
