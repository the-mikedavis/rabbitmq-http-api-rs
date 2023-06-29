use crate::commons::{ExchangeType, PolicyTarget, QueueType};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

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

impl<'a> VirtualHostParams<'a> {
    pub fn named(name: &'a str) -> Self {
        VirtualHostParams {
            name,
            description: None,
            tags: None,
            default_queue_type: None,
            tracing: false,
        }
    }
}

#[derive(Serialize)]
pub struct EnforcedLimitParams<T> {
    pub kind: T,
    pub value: i64,
}

impl<T> EnforcedLimitParams<T> {
    pub fn new(kind: T, value: i64) -> Self {
        EnforcedLimitParams { kind, value }
    }
}

#[derive(Serialize)]
pub struct UserParams<'a> {
    pub name: &'a str,
    pub password_hash: &'a str,
    pub tags: &'a str,
}

pub type XArguments = Option<Map<String, Value>>;

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
        let typ = QueueType::Quorum;
        let args = Self::combined_args(optional_args, &typ);
        Self {
            name,
            queue_type: QueueType::Quorum,
            durable: true,
            auto_delete: false,
            exclusive: false,
            arguments: args,
        }
    }

    pub fn new_stream(name: &'a str, optional_args: XArguments) -> Self {
        let typ = QueueType::Stream;
        let args = Self::combined_args(optional_args, &typ);
        Self {
            name,
            queue_type: QueueType::Stream,
            durable: true,
            auto_delete: false,
            exclusive: false,
            arguments: args,
        }
    }

    pub fn new_durable_classic_queue(name: &'a str, optional_args: XArguments) -> Self {
        let typ = QueueType::Classic;
        let args = Self::combined_args(optional_args, &typ);
        Self {
            name,
            queue_type: QueueType::Classic,
            durable: true,
            auto_delete: false,
            exclusive: false,
            arguments: args,
        }
    }

    pub fn new(
        name: &'a str,
        queue_type: QueueType,
        durable: bool,
        auto_delete: bool,
        optional_args: XArguments,
    ) -> Self {
        let args = Self::combined_args(optional_args, &queue_type);
        Self {
            name,
            queue_type,
            durable,
            auto_delete,
            exclusive: false,
            arguments: args,
        }
    }

    pub fn combined_args(optional_args: XArguments, queue_type: &QueueType) -> XArguments {
        let mut result = Map::<String, Value>::new();
        result.insert("x-queue-type".to_owned(), json!(queue_type));

        if let Some(mut val) = optional_args {
            result.append(&mut val)
        }

        Some(result)
    }
}

#[derive(Debug, Serialize)]
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

pub type RuntimeParameterValue = Map<String, Value>;

#[derive(Serialize, Deserialize)]
pub struct RuntimeParameterDefinition {
    pub name: String,
    pub vhost: String,
    pub component: String,
    pub value: RuntimeParameterValue,
}

pub type PolicyDefinition = Option<Map<String, Value>>;

#[derive(Serialize)]
pub struct PolicyParams<'a> {
    pub vhost: &'a str,
    pub name: &'a str,
    pub pattern: &'a str,
    #[serde(rename(serialize = "apply-to"))]
    pub apply_to: PolicyTarget,
    pub priority: i32,
    pub definition: PolicyDefinition,
}

#[derive(Serialize)]
pub struct Permissions<'a> {
    pub user: &'a str,
    pub vhost: &'a str,
    pub configure: &'a str,
    pub read: &'a str,
    pub write: &'a str,
}
