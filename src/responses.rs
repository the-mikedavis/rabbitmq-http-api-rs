use crate::commons::{BindingDestinationType, PolicyTarget};
use serde::Deserialize;
use serde_aux::prelude::*;
use serde_json::Map;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct VirtualHostMetadata {
    /// Optional tags
    pub tags: Option<Vec<String>>,
    /// Optional description
    pub description: Option<String>,
    /// Default queue type used in this virtual host when clients
    /// do not explicitly specify one
    pub default_queue_type: Option<String>,
}

/// Represents a [RabbitMQ virtual host](https://rabbitmq.com/vhosts.html).
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct VirtualHost {
    /// Virtual host name
    pub name: String,
    /// Optional tags
    pub tags: Option<Vec<String>>,
    /// Optional description
    pub description: Option<String>,
    /// Default queue type used in this virtual host when clients
    /// do not explicitly specify one
    pub default_queue_type: Option<String>,
    /// All virtual host metadata combined
    pub metadata: VirtualHostMetadata,
}

pub type EnforcedLimits = Map<String, serde_json::Value>;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct VirtualHostLimits {
    pub vhost: String,
    #[serde(rename(deserialize = "value"))]
    pub limits: EnforcedLimits,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct UserLimits {
    #[serde(rename(deserialize = "user"))]
    pub username: String,
    #[serde(rename(deserialize = "value"))]
    pub limits: EnforcedLimits,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct User {
    pub name: String,
    pub tags: Vec<String>,
    pub password_hash: String,
}

/// Represents a client connection.
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Connection {
    /// Connection name. Use it to close this connection.
    pub name: String,
    /// To what node the client is connected
    pub node: String,
    /// Connection state
    pub state: String,
    /// What protocol the connection uses
    pub protocol: String,
    /// The name of the authenticated user
    #[serde(rename(deserialize = "user"))]
    pub username: String,
    /// When was this connection opened (a timestamp).
    pub connected_at: u64,
    /// The hostname used to connect.
    #[serde(rename(deserialize = "host"))]
    pub server_hostname: String,
    /// The port used to connect.
    #[serde(rename(deserialize = "port"))]
    pub server_port: u32,
    /// Client hostname.
    #[serde(rename(deserialize = "peer_host"))]
    pub client_hostname: String,
    /// Ephemeral client port.
    #[serde(rename(deserialize = "peer_port"))]
    pub client_port: u32,
    /// Maximum number of channels that can be opened on this connection.
    pub channel_max: u16,
    /// How many channels are opened on this connection.
    #[serde(rename(deserialize = "channels"))]
    pub channel_count: u16,
    /// Client-provided properties (metadata and capabilities).
    pub client_properties: ClientProperties,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ClientProperties {
    pub connection_name: String,
    pub platform: String,
    pub product: String,
    pub version: String,
    pub capabilities: ClientCapabilities,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ClientCapabilities {
    pub authentication_failure_close: bool,
    #[serde(rename(deserialize = "basic.nack"))]
    pub basic_nack: bool,
    #[serde(rename(deserialize = "connection.blocked"))]
    pub connection_blocked: bool,
    #[serde(rename(deserialize = "consumer_cancel_notify"))]
    pub consumer_cancel_notify: bool,
    #[serde(rename(deserialize = "exchange_exchange_bindings"))]
    pub exchange_to_exchange_bindings: bool,
    pub publisher_confirms: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct UserConnection {
    pub name: String,
    pub node: String,
    #[serde(rename(deserialize = "user"))]
    pub username: String,
    pub vhost: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Channel {
    #[serde(rename(deserialize = "number"))]
    pub id: u32,
    pub name: String,
    pub connection_details: ConnectionDetails,
    pub vhost: String,
    pub state: String,
    pub consumer_count: u32,
    #[serde(rename(deserialize = "confirm"))]
    pub has_publisher_confirms_enabled: bool,
    pub prefetch_count: u32,
    pub messages_unacknowledged: u32,
    pub messages_unconfirmed: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ConnectionDetails {
    pub name: String,
    #[serde(rename(deserialize = "peer_host"))]
    pub client_hostname: String,
    #[serde(rename(deserialize = "peer_port"))]
    pub client_port: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ChannelDetails {
    #[serde(rename(deserialize = "number"))]
    pub id: u32,
    pub name: String,
    pub connection_name: String,
    pub node: String,
    #[serde(rename(deserialize = "peer_host"))]
    pub client_hostname: String,
    #[serde(rename(deserialize = "peer_port"))]
    pub client_port: u32,
    #[serde(rename(deserialize = "user"))]
    pub username: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Consumer {
    pub consumer_tag: String,
    pub active: bool,
    #[serde(rename(deserialize = "ack_required"))]
    pub manual_ack: bool,
    pub prefetch_count: u32,
    pub exclusive: bool,
    pub arguments: XArguments,
    #[serde(rename(deserialize = "consumer_timeout"))]
    pub delivery_ack_timeout: u64,
    pub queue: NameAndVirtualHost,
    pub channel_details: ChannelDetails,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct NameAndVirtualHost {
    pub name: String,
    pub vhost: String,
}

pub type XArguments = Map<String, serde_json::Value>;
pub type RuntimeParameterValue = Map<String, serde_json::Value>;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct QueueInfo {
    pub name: String,
    pub vhost: String,
    #[serde(rename(deserialize = "type"))]
    pub queue_type: String,
    pub durable: bool,
    pub auto_delete: bool,
    pub exclusive: bool,
    pub arguments: XArguments,

    pub node: String,
    #[serde(default)]
    pub state: String,
    // only quorum queues and streams will have this
    pub leader: Option<String>,
    pub members: Option<Vec<String>>,
    pub online: Option<Vec<String>>,

    #[serde(default)]
    pub memory: u64,
    #[serde(rename(deserialize = "consumers"))]
    #[serde(default)]
    pub consumer_count: u16,
    #[serde(default)]
    pub consumer_utilisation: f32,
    pub exclusive_consumer_tag: Option<String>,

    pub policy: Option<String>,

    #[serde(default)]
    pub message_bytes: u64,
    #[serde(default)]
    pub message_bytes_persistent: u64,
    #[serde(default)]
    pub message_bytes_ram: u64,
    #[serde(default)]
    pub message_bytes_ready: u64,
    #[serde(default)]
    pub message_bytes_unacknowledged: u64,

    #[serde(rename(deserialize = "messages"))]
    #[serde(default)]
    pub message_count: u64,
    #[serde(rename(deserialize = "messages_persistent"))]
    #[serde(default)]
    pub on_disk_message_count: u64,
    #[serde(rename(deserialize = "messages_ram"))]
    #[serde(default)]
    pub in_memory_message_count: u64,
    #[serde(rename(deserialize = "messages_unacknowledged"))]
    #[serde(default)]
    pub unacknowledged_message_count: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ExchangeInfo {
    pub name: String,
    pub vhost: String,
    #[serde(rename(deserialize = "type"))]
    pub exchange_type: String,
    pub durable: bool,
    pub auto_delete: bool,
    pub arguments: XArguments,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct BindingInfo {
    pub vhost: String,
    pub source: String,
    pub destination: String,
    pub destination_type: BindingDestinationType,
    pub routing_key: String,
    pub arguments: XArguments,
    pub properties_key: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ClusterNode {
    pub name: String,
    pub uptime: u32,
    pub run_queue: u32,
    pub processors: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub os_pid: u32,
    pub fd_total: u32,
    #[serde(rename(deserialize = "proc_total"))]
    pub total_erlang_processes: u32,
    pub sockets_total: u32,
    #[serde(rename(deserialize = "mem_limit"))]
    pub memory_high_watermark: u64,
    #[serde(rename(deserialize = "mem_alarm"))]
    pub has_memory_alarm_in_effect: bool,
    #[serde(rename(deserialize = "disk_free_limit"))]
    pub free_disk_space_low_watermark: u64,
    #[serde(rename(deserialize = "disk_free_alarm"))]
    pub has_free_disk_space_alarm_in_effect: bool,
    pub rates_mode: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct RuntimeParameter {
    pub name: String,
    pub vhost: String,
    pub component: String,
    pub value: RuntimeParameterValue,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ClusterIdentity {
    pub name: String,
}

pub type PolicyDefinition = Option<Map<String, serde_json::Value>>;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Policy {
    pub name: String,
    pub vhost: String,
    pub pattern: String,
    #[serde(rename(deserialize = "apply-to"))]
    pub apply_to: PolicyTarget,
    pub priority: i16,
    pub definition: PolicyDefinition,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub struct Permissions {
    pub user: String,
    pub vhost: String,
    pub configure: String,
    pub read: String,
    pub write: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub enum HealthCheckFailureDetails {
    AlarmCheck(ClusterAlarmCheckDetails),
    NodeIsQuorumCritical(QuorumCriticalityCheckDetails),
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct ClusterAlarmCheckDetails {
    pub reason: String,
    pub alarms: Vec<ResourceAlarm>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct ResourceAlarm {
    pub node: String,
    pub resource: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct QuorumCriticalityCheckDetails {
    pub reason: String,
    pub queues: Vec<QuorumEndangeredQueue>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct QuorumEndangeredQueue {
    pub name: String,
    #[serde(rename(deserialize = "virtual_host"))]
    pub vhost: String,
    #[serde(rename(deserialize = "type"))]
    pub queue_type: String,
}
