use serde::Serialize;
use serde_json::{Map, Value};

#[derive(Serialize)]
pub struct VirtualHostParams<'a> {
    pub name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_queue_type: Option<QueueType>,
    pub tracing: bool,
}

#[derive(Serialize)]
pub struct UserParams<'a> {
    pub name: &'a str,
    pub password_hash: &'a str,
    pub tags: &'a str,
}

type XArguments = Option<Map<String, Value>>;

pub enum QueueType {
    Classic,
    Quorum,
    Stream,
}

impl From<&str> for QueueType {
    fn from(value: &str) -> Self {
        match value {
            "classic" => QueueType::Classic,
            "quorum" => QueueType::Quorum,
            "stream" => QueueType::Stream,
            _ => QueueType::Classic,
        }
    }
}

impl From<String> for QueueType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "classic" => QueueType::Classic,
            "quorum" => QueueType::Quorum,
            "stream" => QueueType::Stream,
            _ => QueueType::Classic,
        }
    }
}

impl Into<String> for QueueType {
    fn into(self) -> String {
        match self {
            QueueType::Classic => "classic".to_owned(),
            QueueType::Quorum => "quorum".to_owned(),
            QueueType::Stream => "stream".to_owned(),
        }
    }
}

impl Serialize for QueueType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match *self {
            QueueType::Classic => "classic",
            QueueType::Quorum => "quorum",
            QueueType::Stream => "stream",
        };
        serializer.serialize_str(&s)
    }
}

/// Exchange types. Most variants are for exchange types included with modern RabbitMQ distributions.
/// For custom types provided by 3rd party plugins, use the `Plugin(String)` variant.
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum ExchangeType {
    /// Fanout exchange
    Fanout,
    /// Topic exchange
    Topic,
    /// Direct exchange
    Direct,
    /// Headers exchange
    Headers,
    /// Consistent hashing (consistent hash) exchange
    ConsistentHashing,
    /// Modulus hash, ships with the 'rabbitmq-sharding' plugin
    ModulusHash,
    /// Random exchange
    Random,
    /// JMS topic exchange
    JmsTopic,
    /// Recent history exchange
    RecentHistory,
    /// All other x-* exchange types, for example, those provided by plugins
    Plugin(String),
}

const EXCHANGE_TYPE_FANOUT: &str = "fanout";
const EXCHANGE_TYPE_TOPIC: &str = "topic";
const EXCHANGE_TYPE_DIRECT: &str = "direct";
const EXCHANGE_TYPE_HEADERS: &str = "headers";
const EXCHANGE_TYPE_CONSISTENT_HASHING: &str = "x-consistent-hash";
const EXCHANGE_TYPE_MODULUS_HASH: &str = "x-modulus-hash";
const EXCHANGE_TYPE_RANDOM: &str = "x-random";
const EXCHANGE_TYPE_JMS_TOPIC: &str = "x-jms-topic";
const EXCHANGE_TYPE_RECENT_HISTORY: &str = "x-recent-history";

impl From<&str> for ExchangeType {
    fn from(value: &str) -> Self {
        match value {
            EXCHANGE_TYPE_FANOUT => ExchangeType::Fanout,
            EXCHANGE_TYPE_TOPIC => ExchangeType::Topic,
            EXCHANGE_TYPE_DIRECT => ExchangeType::Direct,
            EXCHANGE_TYPE_HEADERS => ExchangeType::Headers,
            EXCHANGE_TYPE_CONSISTENT_HASHING => ExchangeType::ConsistentHashing,
            EXCHANGE_TYPE_MODULUS_HASH => ExchangeType::ModulusHash,
            EXCHANGE_TYPE_RANDOM => ExchangeType::Random,
            EXCHANGE_TYPE_JMS_TOPIC => ExchangeType::JmsTopic,
            EXCHANGE_TYPE_RECENT_HISTORY => ExchangeType::RecentHistory,
            other => ExchangeType::Plugin(other.to_owned()),
        }
    }
}

impl From<String> for ExchangeType {
    fn from(value: String) -> Self {
        ExchangeType::from(value.as_str())
    }
}

impl From<ExchangeType> for String {
    fn from(value: ExchangeType) -> String {
        match value {
            ExchangeType::Fanout => EXCHANGE_TYPE_FANOUT.to_owned(),
            ExchangeType::Topic => EXCHANGE_TYPE_TOPIC.to_owned(),
            ExchangeType::Direct => EXCHANGE_TYPE_DIRECT.to_owned(),
            ExchangeType::Headers => EXCHANGE_TYPE_HEADERS.to_owned(),
            ExchangeType::ConsistentHashing => EXCHANGE_TYPE_CONSISTENT_HASHING.to_owned(),
            ExchangeType::ModulusHash => EXCHANGE_TYPE_MODULUS_HASH.to_owned(),
            ExchangeType::Random => EXCHANGE_TYPE_RANDOM.to_owned(),
            ExchangeType::JmsTopic => EXCHANGE_TYPE_JMS_TOPIC.to_owned(),
            ExchangeType::RecentHistory => EXCHANGE_TYPE_RECENT_HISTORY.to_owned(),
            ExchangeType::Plugin(exchange_type) => exchange_type,
        }
    }
}

#[derive(Serialize)]
pub struct QueueParams<'a> {
    pub name: &'a str,
    #[serde(skip_serializing)]
    pub queue_type: QueueType,
    pub durable: bool,
    pub auto_delete: bool,
    pub exclusive: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: XArguments,
}

impl<'a> QueueParams<'a> {
    pub fn new_quorum_queue(name: &'a str, optional_args: XArguments) -> Self {
        let args = Self::combined_args(optional_args, QueueType::Quorum);
        Self {
            name: name,
            queue_type: QueueType::Quorum,
            durable: true,
            auto_delete: false,
            exclusive: false,
            arguments: args,
        }
    }

    pub fn new_stream(name: &'a str, optional_args: XArguments) -> Self {
        let args = Self::combined_args(optional_args, QueueType::Stream);
        Self {
            name: name,
            queue_type: QueueType::Stream,
            durable: true,
            auto_delete: false,
            exclusive: false,
            arguments: args,
        }
    }

    pub fn new_durable_classic_queue(name: &'a str, optional_args: XArguments) -> Self {
        let args = Self::combined_args(optional_args, QueueType::Classic);
        Self {
            name: name,
            queue_type: QueueType::Classic,
            durable: true,
            auto_delete: false,
            exclusive: false,
            arguments: args,
        }
    }

    pub fn new_exclusive_classic_queue(name: &'a str, optional_args: XArguments) -> Self {
        let args = Self::combined_args(optional_args, QueueType::Classic);
        Self {
            name: name,
            queue_type: QueueType::Classic,
            durable: false,
            auto_delete: false,
            exclusive: true,
            arguments: args,
        }
    }

    fn combined_args(optional_args: XArguments, queue_type: QueueType) -> XArguments {
        let mut result = Map::<String, Value>::new();
        result.insert("x-queue-type".to_owned(), Value::String(queue_type.into()));

        match optional_args {
            Some(mut val) => result.append(&mut val),
            None => (),
        }

        Some(result)
    }
}

#[derive(Serialize)]
pub struct ExchangeParams<'a> {
    pub name: &'a str,
    #[serde(rename(serialize = "type"))]
    pub exchange_type: ExchangeType,
    pub durable: bool,
    pub auto_delete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: XArguments,
}

impl<'a> ExchangeParams<'a> {
    pub fn durable(name: &'a str, exchange_type: ExchangeType, optional_args: XArguments) -> Self {
        Self::new(name, exchange_type, true, false, optional_args)
    }

    pub fn fanout(
        name: &'a str,
        durable: bool,
        auto_delete: bool,
        optional_args: XArguments,
    ) -> Self {
        Self::new(
            name,
            ExchangeType::Fanout,
            durable,
            auto_delete,
            optional_args,
        )
    }

    pub fn durable_fanout(name: &'a str, optional_args: XArguments) -> Self {
        Self::new(name, ExchangeType::Fanout, true, false, optional_args)
    }

    pub fn topic(
        name: &'a str,
        durable: bool,
        auto_delete: bool,
        optional_args: XArguments,
    ) -> Self {
        Self::new(
            name,
            ExchangeType::Topic,
            durable,
            auto_delete,
            optional_args,
        )
    }

    pub fn durable_topic(name: &'a str, optional_args: XArguments) -> Self {
        Self::new(name, ExchangeType::Topic, true, false, optional_args)
    }

    pub fn direct(
        name: &'a str,
        durable: bool,
        auto_delete: bool,
        optional_args: XArguments,
    ) -> Self {
        Self::new(
            name,
            ExchangeType::Direct,
            durable,
            auto_delete,
            optional_args,
        )
    }

    pub fn durable_direct(name: &'a str, optional_args: XArguments) -> Self {
        Self::new(name, ExchangeType::Direct, true, false, optional_args)
    }

    pub fn headers(
        name: &'a str,
        durable: bool,
        auto_delete: bool,
        optional_args: XArguments,
    ) -> Self {
        Self::new(
            name,
            ExchangeType::Headers,
            durable,
            auto_delete,
            optional_args,
        )
    }

    pub fn durable_headers(name: &'a str, optional_args: XArguments) -> Self {
        Self::new(name, ExchangeType::Headers, true, false, optional_args)
    }

    pub fn new(
        name: &'a str,
        exchange_type: ExchangeType,
        durable: bool,
        auto_delete: bool,
        optional_args: XArguments,
    ) -> Self {
        Self {
            name,
            exchange_type,
            durable,
            auto_delete,
            arguments: optional_args,
        }
    }
}
