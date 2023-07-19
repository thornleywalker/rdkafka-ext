pub mod traits;
pub mod types;

use std::time::Duration;

use rdkafka::ClientConfig;

use self::{
    traits::{
        ApiTimeoutConfigBuilder, KafkaConfigBuilder, RetriesConfigBuilder, SaslConfigBuilder, Set,
        SslConfigBuilder,
    },
    types::Reset,
};

#[derive(Default)]
pub struct ProducerConfigBuilder {
    config: ClientConfig,
}
impl ProducerConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: ClientConfig::new(),
        }
    }
    pub fn build(self) -> ClientConfig {
        self.config
    }
}
impl Set for ProducerConfigBuilder {
    fn set(&mut self, key: &str, value: impl ToString) {
        self.config.set(key, value.to_string());
    }
}
impl SslConfigBuilder for ProducerConfigBuilder {}
impl SaslConfigBuilder for ProducerConfigBuilder {}
impl KafkaConfigBuilder for ProducerConfigBuilder {}
impl RetriesConfigBuilder for ProducerConfigBuilder {}
impl ProducerConfigBuilder {
    // producer specific ones
}

#[derive(Default)]
pub struct ConsumerConfigBuilder {
    config: ClientConfig,
}
impl ConsumerConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: ClientConfig::new(),
        }
    }
    pub fn build(self) -> ClientConfig {
        self.config
    }
}
impl Set for ConsumerConfigBuilder {
    fn set(&mut self, key: &str, value: impl ToString) {
        self.config.set(key, value.to_string());
    }
}
impl SslConfigBuilder for ConsumerConfigBuilder {}
impl SaslConfigBuilder for ConsumerConfigBuilder {}
impl KafkaConfigBuilder for ConsumerConfigBuilder {}
impl ApiTimeoutConfigBuilder for ConsumerConfigBuilder {}
impl ConsumerConfigBuilder {
    /// The minimum amount of data the server should return for a fetch request. If insufficient
    /// data is available the request will wait for that much data to accumulate before answering
    /// the request. The default setting of 1 byte means that fetch requests are answered as soon
    /// as a single byte of data is available or the fetch request times out waiting for data to
    /// arrive. Setting this to something greater than 1 will cause the server to wait for larger
    /// amounts of data to accumulate which can improve server throughput a bit at the cost of some
    /// additional latency.
    ///
    /// Default: 1
    pub fn fetch_min_bytes(mut self, count: usize) -> Self {
        self.set("fetch.min.bytes", count);
        self
    }
    /// A unique string that identifies the consumer group this consumer belongs to. This property
    /// is required if the consumer uses either the group management functionality by using
    /// `subscribe(topic)` or the Kafka-based offset management strategy.
    ///
    /// Default: null
    pub fn group_id(mut self, group_id: &str) -> Self {
        self.set("group.id", group_id);
        self
    }
    /// The expected time between heartbeats to the consumer coordinator when using Kafka’s group management facilities. Heartbeats are used to ensure that the consumer’s session stays active and to facilitate rebalancing when new consumers join or leave the group. The value must be set lower than `session.timeout.ms`, but typically should be set no higher than 1/3 of that value. It can be adjusted even lower to control the expected time for normal rebalances.
    ///
    /// Default: 3000 (3 seconds)
    pub fn heartbeat_interval(mut self, interval: Duration) -> Self {
        self.set("heartbeat.interval.ms", interval.as_micros());
        self
    }
    /// The maximum amount of data per-partition the server will return. Records are fetched in batches by the consumer. If the first record batch in the first non-empty partition of the fetch is larger than this limit, the batch will still be returned to ensure that the consumer can make progress. The maximum record batch size accepted by the broker is defined via `message.max.bytes` (broker config) or `max.message.bytes` (topic config). See fetch.max.bytes for limiting the consumer request size.
    ///
    /// Default: 1048576 (1 mebibyte)
    pub fn max_partition_fetch_bytes(mut self, count: usize) -> Self {
        self.set("max.partition.fetch.bytes", count);
        self
    }
    /// The timeout used to detect client failures when using Kafka’s group management facility. The client sends periodic heartbeats to indicate its liveness to the broker. If no heartbeats are received by the broker before the expiration of this session timeout, then the broker will remove this client from the group and initiate a rebalance. Note that the value must be in the allowable range as configured in the broker configuration by `group.min.session.timeout.ms` and `group.max.session.timeout.ms`.
    ///
    /// Default: 45000 (45 seconds)
    pub fn session_timeout(mut self, timeout: Duration) -> Self {
        self.set("session.timeout.ms", timeout.as_micros());
        self
    }

    // more ssl stuff that I don't care about
    //
    //
    /// Allow automatic topic creation on the broker when subscribing to or assigning a topic. A topic being subscribed to will be automatically created only if the broker allows for it using auto.create.topics.enable broker configuration. This configuration must be set to false when using brokers older than 0.11.0
    ///
    /// Default: true
    pub fn allow_auto_create_topics(mut self, allow: bool) -> Self {
        self.set("allow.auto.create.topics", allow);
        self
    }
    /// What to do when there is no initial offset in Kafka or if the current offset does not exist any more on the server (e.g. because that data has been deleted):
    ///
    /// - earliest: automatically reset the offset to the earliest offset
    /// - latest: automatically reset the offset to the latest offset
    /// - none: throw exception to the consumer if no previous offset is found for the consumer’s group
    /// - anything else: throw exception to the consumer.
    ///
    /// Default: latest
    pub fn auto_offset_reset(mut self, reset: Reset) -> Self {
        self.set("auto.offset.reset", reset);
        self
    }
    /// Close idle connections after the number of milliseconds specified by this config.
    ///
    /// Default: 540000 (9 minutes)
    pub fn connections_max_idle(mut self, idle: Duration) -> Self {
        self.set("connections.max.idle.ms", idle.as_micros());
        self
    }
    /// Specifies the timeout (in milliseconds) for client APIs. This configuration is used as the default timeout for all client operations that do not specify a timeout parameter.
    ///
    /// Default: 60000 (1 minute)
    pub fn default_api_timout(mut self, timeout: Duration) -> Self {
        self.set("default.api.timeout.ms", timeout.as_micros());
        self
    }
    /// If true the consumer’s offset will be periodically committed in the background.
    ///
    /// Default: true
    pub fn enable_auto_commit(mut self, commit: bool) -> Self {
        self.set("enable.auto.commit", commit);
        self
    }
    /// Whether internal topics matching a subscribed pattern should be excluded from the subscription. It is always possible to explicitly subscribe to an internal topic.
    ///
    /// Default: true
    pub fn exclude_internal_topics(mut self, exclude: bool) -> Self {
        self.set("exclude.internal.topics", exclude);
        self
    }
    /// The maximum amount of data the server should return for a fetch request. Records are fetched in batches by the consumer, and if the first record batch in the first non-empty partition of the fetch is larger than this value, the record batch will still be returned to ensure that the consumer can make progress. As such, this is not a absolute maximum. The maximum record batch size accepted by the broker is defined via `message.max.bytes` (broker config) or `max.message.bytes` (topic config). Note that the consumer performs multiple fetches in parallel.
    ///
    /// Defualt: 52428800 (50 mebibytes)
    pub fn fetch_max_bytes(mut self, count: usize) -> Self {
        self.set("fetch.max.bytes", count);
        self
    }
    /// A unique identifier of the consumer instance provided by the end user. Only non-empty strings are permitted. If set, the consumer is treated as a static member, which means that only one instance with this ID is allowed in the consumer group at any time. This can be used in combination with a larger session timeout to avoid group rebalances caused by transient unavailability (e.g. process restarts). If not set, the consumer will join the group as a dynamic member, which is the traditional behavior.
    ///
    /// Default: null
    pub fn group_instance_id(mut self, id: &str) -> Self {
        self.set("group.instance.id", id);
        self
    }
    /* TODO: isolation.level */
    /// The maximum delay between invocations of poll() when using consumer group management. This places an upper bound on the amount of time that the consumer can be idle before fetching more records. If poll() is not called before expiration of this timeout, then the consumer is considered failed and the group will rebalance in order to reassign the partitions to another member. For consumers using a non-null `group.instance.id` which reach this timeout, partitions will not be immediately reassigned. Instead, the consumer will stop sending heartbeats and partitions will be reassigned after expiration of `session.timeout.ms`. This mirrors the behavior of a static consumer which has shutdown.
    ///
    /// Default: 300000 (5 minutes)
    pub fn max_poll_interval(mut self, interval: Duration) -> Self {
        self.set("max.poll.interval.ms", interval.as_micros());
        self
    }
    /// The maximum number of records returned in a single call to poll(). Note, that `max.poll.records` does not impact the underlying fetching behavior. The consumer will cache the records from each fetch request and returns them incrementally from each poll.
    ///
    /// Default: 500
    pub fn max_poll_records(mut self, count: usize) -> Self {
        self.set("max.poll.records", count);
        self
    }
    /* TODO: more after here */
}

#[derive(Default)]
pub struct AdminConfigBuilder {
    config: ClientConfig,
}
impl AdminConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: ClientConfig::new(),
        }
    }
    pub fn build(self) -> ClientConfig {
        self.config
    }
}
impl Set for AdminConfigBuilder {
    fn set(&mut self, key: &str, value: impl ToString) {
        self.config.set(key, value.to_string());
    }
}
impl SslConfigBuilder for AdminConfigBuilder {}
impl SaslConfigBuilder for AdminConfigBuilder {}
impl KafkaConfigBuilder for AdminConfigBuilder {}
impl ApiTimeoutConfigBuilder for AdminConfigBuilder {}
impl RetriesConfigBuilder for AdminConfigBuilder {}
impl AdminConfigBuilder {
    // admin specific configs
    // host resolver?
}
