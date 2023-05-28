use serde::Serialize;
use serde_json::{Value, Map};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<&'a str>>,
}

type XArguments = Option<Map<String, Value>>;

pub enum QueueType {
    Classic,
    Quorum,
    Stream
}

impl From<&str> for QueueType {
    fn from(value: &str) -> Self {
        match value {
            "classic" => QueueType::Classic,
            "quorum" => QueueType::Quorum,
            "stream" => QueueType::Stream,
            _ => QueueType::Classic
        }
    }
}

impl From<String> for QueueType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "classic" => QueueType::Classic,
            "quorum" => QueueType::Quorum,
            "stream" => QueueType::Stream,
            _ => QueueType::Classic
        }
    }
}

impl Into<String> for QueueType {
    fn into(self) -> String {
        match self {
            QueueType::Classic => "classic".to_owned(),
            QueueType::Quorum => "quorum".to_owned(),
            QueueType::Stream => "stream".to_owned()
        }
    }
}

impl Serialize for QueueType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let s = match *self {
            QueueType::Classic => "classic",
            QueueType::Quorum => "quorum",
            QueueType::Stream => "stream"
        };
        serializer.serialize_str(&s)
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
    pub arguments: XArguments
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
            arguments: args
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
            arguments: args
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
            arguments: args
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
            arguments: args
        }
    }

    fn combined_args(optional_args: XArguments, queue_type: QueueType) -> XArguments {
        let mut result = Map::<String, Value>::new();
        result.insert("x-queue-type".to_owned(), Value::String(queue_type.into()));

        match optional_args {
            Some(mut val) => result.append(&mut val),
            None => ()
        }

        Some(result)
    }
}


#[derive(Serialize)]
pub struct ExchangeParams<'a> {
    pub name: &'a str,
    #[serde(rename(serialize = "type"))]
    pub exchange_type: &'a str,
    pub durable: bool,
    pub auto_delete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: XArguments
}

impl<'a> ExchangeParams<'a> {
    pub fn durable(name: &'a str, exchange_type: &'a str, optional_args: XArguments) -> Self {
        Self::new(name, exchange_type, true, false, optional_args)
    }

    pub fn fanout(name: &'a str, durable: bool, auto_delete: bool, optional_args: XArguments) -> Self {
        Self::new(name, "fanout", durable, auto_delete, optional_args)
    }

    pub fn durable_fanout(name: &'a str, optional_args: XArguments) -> Self {
        Self::new(name, "fanout", true, false, optional_args)
    }

    pub fn topic(name: &'a str, durable: bool, auto_delete: bool, optional_args: XArguments) -> Self {
        Self::new(name, "topic", durable, auto_delete, optional_args)
    }

    pub fn durable_topic(name: &'a str, optional_args: XArguments) -> Self {
        Self::new(name, "topic", true, false, optional_args)
    }

    pub fn direct(name: &'a str, durable: bool, auto_delete: bool, optional_args: XArguments) -> Self {
        Self::new(name, "direct", durable, auto_delete, optional_args)
    }

    pub fn durable_direct(name: &'a str, optional_args: XArguments) -> Self {
        Self::new(name, "direct", true, false, optional_args)
    }

    pub fn headers(name: &'a str, durable: bool, auto_delete: bool, optional_args: XArguments) -> Self {
        Self::new(name, "headers", durable, auto_delete, optional_args)
    }

    pub fn durable_headers(name: &'a str, optional_args: XArguments) -> Self {
        Self::new(name, "headers", true, false, optional_args)
    }

    pub fn new(name: &'a str, exchange_type: &'a str, durable: bool, auto_delete: bool, optional_args: XArguments) -> Self {
        Self {
            name,
            exchange_type,
            durable,
            auto_delete,
            arguments: optional_args
        }
    }
}