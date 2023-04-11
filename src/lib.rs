pub mod builders;

use builders::{traits::KafkaConfigBuilder, ConsumerConfigBuilder, ProducerConfigBuilder};
use futures::{Stream, StreamExt};
use rdkafka::{
    consumer::StreamConsumer,
    error::KafkaError,
    message::{BorrowedHeaders, BorrowedMessage},
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
    ClientConfig, Message, Timestamp,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Topic: Clone {
    type Payload: Serialize + DeserializeOwned;

    fn topic_str(&self) -> String;
}

pub struct TypedMessage<'a, T> {
    borrowed_message: BorrowedMessage<'a>,
    topic: T,
}
impl<'a, T: Topic> TypedMessage<'a, T> {
    pub fn key(&self) -> Option<&[u8]> {
        self.borrowed_message.key()
    }
    pub fn payload(&self) -> Option<T::Payload> {
        self.borrowed_message
            .payload()
            .map(|val| serde_json::from_slice(val).unwrap())
    }
    pub fn topic(&self) -> &T {
        &self.topic
    }
    pub fn partition(&self) -> i32 {
        self.borrowed_message.partition()
    }
    pub fn offset(&self) -> i64 {
        self.borrowed_message.offset()
    }
    pub fn timestamp(&self) -> Timestamp {
        self.borrowed_message.timestamp()
    }
    pub fn headers(&self) -> Option<&BorrowedHeaders> {
        self.borrowed_message.headers()
    }
}

pub struct TypedProducer {
    inner: FutureProducer,
}
impl TypedProducer {
    pub fn new(config: ClientConfig) -> Self {
        Self {
            inner: config.create().unwrap(),
        }
    }
    pub async fn send<T: Topic>(
        &self,
        topic: &T,
        payload: &T::Payload,
        key: Option<String>,
        timeout: impl Into<Timeout>,
    ) {
        let bytes = serde_json::to_vec(payload).unwrap();
        let topic_string = topic.topic_str();

        let record = FutureRecord::to(&topic_string).payload(&bytes);
        let record = if let Some(ref key) = key {
            record.key(key)
        } else {
            record
        };

        self.inner.send(record, timeout).await.unwrap();
    }
}

pub struct TypedConsumer<T> {
    inner: StreamConsumer,
    topic: T,
}

impl<T: Topic> TypedConsumer<T> {
    pub fn new(client_config: ClientConfig, topic: T) -> Self {
        let inner = client_config.create().unwrap();

        Self { inner, topic }
    }
    pub fn topic(&self) -> &T {
        &self.topic
    }
    pub async fn recv(&self) -> Result<TypedMessage<'_, T>, KafkaError> {
        Ok(TypedMessage {
            borrowed_message: self.inner.recv().await?,
            topic: self.topic.clone(),
        })
    }
    pub async fn stream(&self) -> impl Stream<Item = Result<TypedMessage<'_, T>, KafkaError>> + '_ {
        self.inner.stream().map(|val| {
            val.map(|borrowed_message| TypedMessage {
                borrowed_message,
                topic: self.topic.clone(),
            })
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Update {
    Thing1,
    Thing2,
}

#[derive(Clone)]
struct SessionTopic {
    id: String,
}
impl Topic for SessionTopic {
    type Payload = Update;

    fn topic_str(&self) -> String {
        format!("session:{}", self.id)
    }
}

async fn _consumer_example() {
    let config = ConsumerConfigBuilder::new()
        .bootstrap_servers(&["localhost:9092"])
        .allow_auto_create_topics(true)
        .group_id(&format!("user:{}", 123))
        .build();

    let consumer = TypedConsumer::new(
        config,
        SessionTopic {
            id: "asdflkj".to_string(),
        },
    );

    let mut stream = consumer.stream().await;
    while let Some(Ok(message)) = stream.next().await {
        if let Some(payload) = message.payload() {
            match payload {
                Update::Thing1 => println!("Do thing 1"),
                Update::Thing2 => println!("Do thing 2"),
            }
        }
    }
}

async fn _producer_example() {
    let config = ProducerConfigBuilder::new()
        .bootstrap_servers(&["localhost:9092"])
        .client_id("client")
        .build();

    let producer = TypedProducer::new(config);

    let topic = SessionTopic {
        id: "s2d54f".to_string(),
    };

    producer.send(&topic, &Update::Thing1, None, None).await;
}
