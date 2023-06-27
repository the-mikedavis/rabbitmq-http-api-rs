use crate::{
    commons::{BindingDestinationType, UserLimitTarget, VirtualHostLimitTarget},
    requests::{
        EnforcedLimitParams, ExchangeParams, Permissions, PolicyParams, QueueParams,
        RuntimeParameterDefinition, UserParams, VirtualHostParams, XArguments,
    },
    responses::{self, BindingInfo},
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::blocking::Client as HttpClient;
use serde::Serialize;
use serde_json::{json, Map, Value};
use std::{collections::HashMap, fmt::Display};

use thiserror::Error;

type HttpClientResponse = reqwest::blocking::Response;

#[derive(Error, Debug)]
pub enum Error {
    #[error("encountered an error when performing an HTTP request")]
    RequestError(#[from] reqwest::Error),
    #[error("API responded with a client error: status code of {0}")]
    ClientErrorResponse(u16, HttpClientResponse),
    #[error("API responded with a server error: status code of {0}")]
    ServerErrorResponse(u16, HttpClientResponse),
    #[error("Health check failed: resource alarms are in effect")]
    HealthCheckFailed(responses::HealthCheckFailureDetails),
    #[error("Could not find the requested resource")]
    NotFound(),
    #[error("Can't delete a binding: multiple matching bindings found")]
    ManyMatchingBindings(),
    #[error("an unspecified error")]
    Other,
}

pub type Result<T> = std::result::Result<T, Error>;

/// A client for the [RabbitMQ HTTP API](https://rabbitmq.com/management.html#http-api).
///
/// Most functions provided by this type represent various HTTP API operations.
/// For example,
///
///  * the [`Client::get_queue_info`] function corresponds to the `GET /api/queues/{vhost}/{name}` endpoint
///  * the [`Client::list_user_connections`] function corresponds to the `GET /api/connections/username/{username}` endpoint
///
/// and so on.
///
/// Example
/// ```rust
/// let endpoint = "http://localhost:15672/api/";
/// let username = "username";
/// let password = "password";
/// let rc = Client::new_with_basic_auth_credentials(&endpoint, &username, &password);
/// // list cluster nodes
/// rc.list_nodes()?;
/// // list user connections
/// rc.list_connections()?;
/// // fetch information and metrics of a specific queue
/// rc.get_queue_info("/", "qq.1")?;
/// ```
pub struct Client<'a> {
    endpoint: &'a str,
    username: &'a str,
    password: &'a str,
}

impl<'a> Client<'a> {
    /// Instantiates a client that will use Basic HTTP Auth for authentication.
    ///
    /// Example
    /// ```rust
    /// let endpoint = "http://localhost:15672/api/";
    /// let username = "username";
    /// let password = "password";
    /// let rc = Client::new_with_basic_auth_credentials(&endpoint, &username, &password);
    /// ```
    pub fn new_with_basic_auth_credentials(
        endpoint: &'a str,
        username: &'a str,
        password: &'a str,
    ) -> Self {
        Self {
            endpoint,
            username,
            password,
        }
    }

    /// Lists cluster nodes.
    pub fn list_nodes(&self) -> Result<Vec<responses::ClusterNode>> {
        let response = self.http_get("nodes")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::ClusterNode>>()
            .map_err(Error::from)
    }

    /// Lists virtual hosts in the cluster.
    pub fn list_vhosts(&self) -> Result<Vec<responses::VirtualHost>> {
        let response = self.http_get("vhosts")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::VirtualHost>>()
            .map_err(Error::from)
    }

    /// Lists users in the internal database.
    pub fn list_users(&self) -> Result<Vec<responses::User>> {
        let response = self.http_get("users")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::User>>()
            .map_err(Error::from)
    }

    /// Lists all client connections across the cluster.
    pub fn list_connections(&self) -> Result<Vec<responses::Connection>> {
        let response = self.http_get("connections")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::Connection>>()
            .map_err(Error::from)
    }

    /// Lists all connections of a specific user.
    pub fn list_user_connections(&self, username: &str) -> Result<Vec<responses::UserConnection>> {
        let response = self.http_get(&format!(
            "connections/username/{}",
            self.percent_encode(username)
        ))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::UserConnection>>()
            .map_err(Error::from)
    }

    pub fn list_channels(&self) -> Result<Vec<responses::Channel>> {
        let response = self.http_get("channels")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::Channel>>()
            .map_err(Error::from)
    }

    pub fn list_queues(&self) -> Result<Vec<responses::QueueInfo>> {
        let response = self.http_get("queues")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::QueueInfo>>()
            .map_err(Error::from)
    }

    pub fn list_queues_in(&self, virtual_host: &str) -> Result<Vec<responses::QueueInfo>> {
        let response = self.http_get(&format!("queues/{}", self.percent_encode(virtual_host)))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::QueueInfo>>()
            .map_err(Error::from)
    }

    pub fn list_exchanges(&self) -> Result<Vec<responses::ExchangeInfo>> {
        let response = self.http_get("exchanges")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::ExchangeInfo>>()
            .map_err(Error::from)
    }

    pub fn list_exchanges_in(&self, virtual_host: &str) -> Result<Vec<responses::ExchangeInfo>> {
        let response =
            self.http_get(&format!("exchanges/{}", self.percent_encode(virtual_host)))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::ExchangeInfo>>()
            .map_err(Error::from)
    }

    pub fn list_bindings(&self) -> Result<Vec<responses::BindingInfo>> {
        let response = self.http_get("bindings")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::BindingInfo>>()
            .map_err(Error::from)
    }

    pub fn list_bindings_in(&self, virtual_host: &str) -> Result<Vec<responses::BindingInfo>> {
        let response = self.http_get(&format!("bindings/{}", self.percent_encode(virtual_host)))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::BindingInfo>>()
            .map_err(Error::from)
    }

    pub fn list_queue_bindings(
        &self,
        virtual_host: &str,
        queue: &str,
    ) -> Result<Vec<responses::BindingInfo>> {
        let response = self.http_get(&format!(
            "queues/{}/{}/bindings",
            self.percent_encode(virtual_host),
            self.percent_encode(queue)
        ))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::BindingInfo>>()
            .map_err(Error::from)
    }

    pub fn list_exchange_bindings_with_source(
        &self,
        virtual_host: &str,
        exchange: &str,
    ) -> Result<Vec<responses::BindingInfo>> {
        self.list_exchange_bindings_with_source_or_destination(
            virtual_host,
            exchange,
            BindindVertex::Source,
        )
    }

    pub fn list_exchange_bindings_with_destination(
        &self,
        virtual_host: &str,
        exchange: &str,
    ) -> Result<Vec<responses::BindingInfo>> {
        self.list_exchange_bindings_with_source_or_destination(
            virtual_host,
            exchange,
            BindindVertex::Destination,
        )
    }

    pub fn list_consumers(&self) -> Result<Vec<responses::Consumer>> {
        let response = self.http_get("consumers")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::Consumer>>()
            .map_err(Error::from)
    }

    pub fn list_consumers_in(&self, virtual_host: &str) -> Result<Vec<responses::Consumer>> {
        let response = self.http_get(&format!("consumers/{}", virtual_host))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::Consumer>>()
            .map_err(Error::from)
    }

    pub fn get_node_info(&self, name: &str) -> Result<responses::ClusterNode> {
        let response = self.http_get(&format!("nodes/{}", name))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<responses::ClusterNode>()
            .map_err(Error::from)
    }

    pub fn get_vhost(&self, name: &str) -> Result<responses::VirtualHost> {
        let response = self.http_get(&format!("vhosts/{}", self.percent_encode(name)))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<responses::VirtualHost>()
            .map_err(Error::from)
    }

    pub fn get_user(&self, name: &str) -> Result<responses::User> {
        let response = self.http_get(&format!("users/{}", self.percent_encode(name)))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2.json::<responses::User>().map_err(Error::from)
    }

    pub fn get_queue_info(&self, virtual_host: &str, name: &str) -> Result<responses::QueueInfo> {
        let response = self.http_get(&format!(
            "queues/{}/{}",
            self.percent_encode(virtual_host),
            self.percent_encode(name)
        ))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<responses::QueueInfo>()
            .map_err(Error::from)
    }

    pub fn get_exchange_info(
        &self,
        virtual_host: &str,
        name: &str,
    ) -> Result<responses::ExchangeInfo> {
        let response = self.http_get(&format!(
            "exchanges/{}/{}",
            self.percent_encode(virtual_host),
            self.percent_encode(name)
        ))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<responses::ExchangeInfo>()
            .map_err(Error::from)
    }

    pub fn create_vhost(&self, params: &VirtualHostParams) -> Result<()> {
        self.update_vhost(params)
    }

    pub fn update_vhost(&self, params: &VirtualHostParams) -> Result<()> {
        let response = self.http_put(
            &format!("vhosts/{}", self.percent_encode(params.name)),
            params,
        )?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn create_user(&self, params: &UserParams) -> Result<()> {
        let response = self.http_put(
            &format!("users/{}", self.percent_encode(params.name)),
            params,
        )?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn declare_permissions(&self, params: &Permissions) -> Result<()> {
        let response = self.http_put(
            // /api/permissions/vhost/user
            &format!(
                "permissions/{}/{}",
                self.percent_encode(params.vhost),
                self.percent_encode(params.user)
            ),
            params,
        )?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn delete_permissions(&self, vhost: &str, user: &str) -> Result<()> {
        let response = self.http_delete(&format!(
            "permissions/{}/{}",
            self.percent_encode(vhost),
            self.percent_encode(user)
        ))?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn declare_queue(&self, virtual_host: &str, params: &QueueParams) -> Result<()> {
        let response = self.http_put(
            &format!(
                "queues/{}/{}",
                self.percent_encode(virtual_host),
                self.percent_encode(params.name)
            ),
            params,
        )?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn declare_exchange(&self, virtual_host: &str, params: &ExchangeParams) -> Result<()> {
        let response = self.http_put(
            &format!(
                "exchanges/{}/{}",
                self.percent_encode(virtual_host),
                self.percent_encode(params.name)
            ),
            params,
        )?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn bind_queue(
        &self,
        virtual_host: &str,
        queue: &str,
        exchange: &str,
        routing_key: Option<&str>,
        arguments: XArguments,
    ) -> Result<()> {
        let mut body = Map::<String, Value>::new();
        if let Some(rk) = routing_key {
            body.insert("routing_key".to_owned(), json!(rk));
        }
        if let Some(args) = arguments {
            body.insert("arguments".to_owned(), json!(args));
        }

        let path = format!(
            "bindings/{}/e/{}/q/{}",
            self.percent_encode(virtual_host),
            self.percent_encode(exchange),
            self.percent_encode(queue)
        );
        let response = self.http_post(&path, &body)?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn bind_exchange(
        &self,
        virtual_host: &str,
        destination: &str,
        source: &str,
        routing_key: Option<&str>,
        arguments: XArguments,
    ) -> Result<()> {
        let mut body = Map::<String, Value>::new();
        if let Some(rk) = routing_key {
            body.insert("routing_key".to_owned(), json!(rk));
        }
        if let Some(args) = arguments {
            body.insert("arguments".to_owned(), json!(args));
        }

        let path = format!(
            "bindings/{}/e/{}/e/{}",
            self.percent_encode(virtual_host),
            self.percent_encode(source),
            self.percent_encode(destination)
        );
        let response = self.http_post(&path, &body)?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn delete_vhost(&self, virtual_host: &str) -> Result<()> {
        let response =
            self.http_delete(&format!("vhosts/{}", self.percent_encode(virtual_host)))?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn delete_user(&self, username: &str) -> Result<()> {
        let response = self.http_delete(&format!("users/{}", self.percent_encode(username)))?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn delete_queue(&self, virtual_host: &str, name: &str) -> Result<()> {
        let response = self.http_delete(&format!(
            "queues/{}/{}",
            self.percent_encode(virtual_host),
            self.percent_encode(name)
        ))?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn delete_exchange(&self, virtual_host: &str, name: &str) -> Result<()> {
        let response = self.http_delete(&format!(
            "exchanges/{}/{}",
            self.percent_encode(virtual_host),
            self.percent_encode(name)
        ))?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn delete_binding(
        &self,
        virtual_host: &str,
        source: &str,
        destination: &str,
        destination_type: BindingDestinationType,
        routing_key: &str,
        arguments: XArguments,
    ) -> Result<HttpClientResponse> {
        let args = arguments.unwrap();

        // to delete a binding, we need properties, that we can get from the server
        // so we search for the binding before deleting it
        let bindings = match destination_type {
            BindingDestinationType::Queue => self.list_queue_bindings(virtual_host, destination)?,
            BindingDestinationType::Exchange => {
                self.list_exchange_bindings_with_destination(virtual_host, destination)?
            }
        };

        let bs: Vec<&BindingInfo> = bindings
            .iter()
            .filter(|b| b.source == source && b.routing_key == routing_key && b.arguments == args)
            .collect();
        match bs.len() {
            0 => Err(Error::NotFound()),
            1 => {
                let first_key = bs.first().unwrap().properties_key.as_str();
                let response = self.http_delete(&format!(
                    // /api/bindings/vhost/e/exchange/[eq]/destination/props
                    "bindings/{}/e/{}/{}/{}/{}",
                    self.percent_encode(virtual_host),
                    self.percent_encode(source),
                    destination_type.path_appreviation(),
                    self.percent_encode(destination),
                    self.percent_encode(first_key),
                ))?;
                self.ok_or_status_code_error(response)
            }
            _ => Err(Error::ManyMatchingBindings()),
        }
    }

    pub fn purge_queue(&self, virtual_host: &str, name: &str) -> Result<()> {
        let response = self.http_delete(&format!(
            "queues/{}/{}/contents",
            self.percent_encode(virtual_host),
            self.percent_encode(name)
        ))?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn list_runtime_parameters(&self) -> Result<Vec<responses::RuntimeParameter>> {
        let response = self.http_get("parameters")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::RuntimeParameter>>()
            .map_err(Error::from)
    }

    pub fn list_runtime_parameters_of_component(
        &self,
        component: &str,
    ) -> Result<Vec<responses::RuntimeParameter>> {
        let path = format!("parameters/{}", self.percent_encode(component));
        let response = self.http_get(&path)?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::RuntimeParameter>>()
            .map_err(Error::from)
    }

    pub fn list_runtime_parameters_of_component_in(
        &self,
        component: &str,
        vhost: &str,
    ) -> Result<Vec<responses::RuntimeParameter>> {
        let path = format!(
            "parameters/{}/{}",
            self.percent_encode(component),
            self.percent_encode(vhost)
        );
        let response = self.http_get(&path)?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::RuntimeParameter>>()
            .map_err(Error::from)
    }

    pub fn get_runtime_parameter(
        &self,
        component: &str,
        vhost: &str,
        name: &str,
    ) -> Result<responses::RuntimeParameter> {
        let path = format!(
            "parameters/{}/{}/{}",
            self.percent_encode(component),
            self.percent_encode(vhost),
            self.percent_encode(name)
        );
        let response = self.http_get(&path)?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<responses::RuntimeParameter>()
            .map_err(Error::from)
    }

    pub fn upsert_runtime_parameter(&self, param: &RuntimeParameterDefinition) -> Result<()> {
        let path = format!(
            "parameters/{}/{}/{}",
            self.percent_encode(&param.component),
            self.percent_encode(&param.vhost),
            self.percent_encode(&param.name)
        );
        let response = self.http_put(&path, &param)?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn clear_runtime_parameter(&self, component: &str, vhost: &str, name: &str) -> Result<()> {
        let path = format!(
            "parameters/{}/{}/{}",
            self.percent_encode(component),
            self.percent_encode(vhost),
            self.percent_encode(name)
        );
        let response = self.http_delete(&path)?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn clear_all_runtime_parameters(&self) -> Result<()> {
        let params = self.list_runtime_parameters()?;
        for rp in params {
            self.clear_runtime_parameter(&rp.component, &rp.vhost, &rp.name)?
        }
        Ok(())
    }

    pub fn clear_all_runtime_parameters_of_component(&self, component: &str) -> Result<()> {
        let params = self.list_runtime_parameters_of_component(component)?;
        for rp in params {
            self.clear_runtime_parameter(&rp.component, &rp.vhost, &rp.name)?
        }
        Ok(())
    }

    pub fn set_user_limit(
        &self,
        username: &str,
        limit: EnforcedLimitParams<UserLimitTarget>,
    ) -> Result<()> {
        let path = format!("user-limits/{}/{}", username, String::from(limit.kind));

        let mut body = Map::<String, Value>::new();
        body.insert("value".to_owned(), json!(limit.value));

        let response = self.http_put(&path, &body)?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn clear_user_limit(&self, username: &str, kind: UserLimitTarget) -> Result<()> {
        let path = format!("user-limits/{}/{}", username, String::from(kind));

        let response = self.http_delete(&path)?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn list_all_user_limits(&self) -> Result<Vec<responses::UserLimits>> {
        let response = self.http_get("user-limits")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::UserLimits>>()
            .map_err(Error::from)
    }

    pub fn list_user_limits(&self, username: &str) -> Result<Vec<responses::UserLimits>> {
        let path = format!("user-limits/{}", username);
        let response = self.http_get(&path)?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::UserLimits>>()
            .map_err(Error::from)
    }

    pub fn set_vhost_limit(
        &self,
        vhost: &str,
        limit: EnforcedLimitParams<VirtualHostLimitTarget>,
    ) -> Result<()> {
        let path = format!(
            "vhost-limits/{}/{}",
            self.percent_encode(vhost),
            String::from(limit.kind)
        );

        let mut body = Map::<String, Value>::new();
        body.insert("value".to_owned(), json!(limit.value));

        let response = self.http_put(&path, &body)?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn clear_vhost_limit(&self, vhost: &str, kind: VirtualHostLimitTarget) -> Result<()> {
        let path = format!(
            "vhost-limits/{}/{}",
            self.percent_encode(vhost),
            String::from(kind)
        );

        let response = self.http_delete(&path)?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn list_all_vhost_limits(&self) -> Result<Vec<responses::VirtualHostLimits>> {
        let response = self.http_get("vhost-limits")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::VirtualHostLimits>>()
            .map_err(Error::from)
    }

    pub fn list_vhost_limits(&self, vhost: &str) -> Result<Vec<responses::VirtualHostLimits>> {
        let path = format!("vhost-limits/{}", self.percent_encode(vhost));
        let response = self.http_get(&path)?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::VirtualHostLimits>>()
            .map_err(Error::from)
    }

    pub fn get_cluster_name(&self) -> Result<responses::ClusterIdentity> {
        let response = self.http_get("cluster-name")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<responses::ClusterIdentity>()
            .map_err(Error::from)
    }

    pub fn set_cluster_name(&self, new_name: &str) -> Result<()> {
        let mut map = HashMap::new();
        map.insert("name", new_name);

        let response = self.http_put("cluster-name", &map)?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn get_policy(&self, vhost: &str, name: &str) -> Result<responses::Policy> {
        let response = self.http_get(&format!(
            "policies/{}/{}",
            self.percent_encode(vhost),
            self.percent_encode(name)
        ))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2.json::<responses::Policy>().map_err(Error::from)
    }

    pub fn list_policies(&self) -> Result<Vec<responses::Policy>> {
        let response = self.http_get("policies")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::Policy>>()
            .map_err(Error::from)
    }

    pub fn list_policies_in(&self, vhost: &str) -> Result<Vec<responses::Policy>> {
        let response = self.http_get(&format!("policies/{}", self.percent_encode(vhost)))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::Policy>>()
            .map_err(Error::from)
    }

    pub fn declare_policy(&self, params: &PolicyParams) -> Result<()> {
        let response = self.http_put(
            &format!(
                "policies/{}/{}",
                self.percent_encode(params.vhost),
                self.percent_encode(params.name)
            ),
            params,
        )?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn delete_policy(&self, vhost: &str, name: &str) -> Result<()> {
        let response = self.http_delete(&format!(
            "policies/{}/{}",
            self.percent_encode(vhost),
            self.percent_encode(name)
        ))?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn get_operator_policy(&self, vhost: &str, name: &str) -> Result<responses::Policy> {
        let response = self.http_get(&format!(
            "operator-policies/{}/{}",
            self.percent_encode(vhost),
            self.percent_encode(name)
        ))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2.json::<responses::Policy>().map_err(Error::from)
    }

    pub fn list_operator_policies(&self) -> Result<Vec<responses::Policy>> {
        let response = self.http_get("operator-policies")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::Policy>>()
            .map_err(Error::from)
    }

    pub fn list_operator_policies_in(&self, vhost: &str) -> Result<Vec<responses::Policy>> {
        let response =
            self.http_get(&format!("operator-policies/{}", self.percent_encode(vhost)))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::Policy>>()
            .map_err(Error::from)
    }

    pub fn declare_operator_policy(&self, params: &PolicyParams) -> Result<()> {
        let response = self.http_put(
            &format!(
                "operator-policies/{}/{}",
                self.percent_encode(params.vhost),
                self.percent_encode(params.name)
            ),
            params,
        )?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn delete_operator_policy(&self, vhost: &str, name: &str) -> Result<()> {
        let response = self.http_delete(&format!(
            "operator-policies/{}/{}",
            self.percent_encode(vhost),
            self.percent_encode(name)
        ))?;
        self.ok_or_status_code_error(response)?;
        Ok(())
    }

    pub fn list_permissions(&self) -> Result<Vec<responses::Permissions>> {
        let response = self.http_get("permissions")?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::Permissions>>()
            .map_err(Error::from)
    }

    pub fn list_permissions_in(&self, vhost: &str) -> Result<Vec<responses::Permissions>> {
        let response = self.http_get(&format!(
            "vhosts/{}/permissions",
            self.percent_encode(vhost)
        ))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::Permissions>>()
            .map_err(Error::from)
    }

    pub fn list_permissions_of(&self, user: &str) -> Result<Vec<responses::Permissions>> {
        let response =
            self.http_get(&format!("users/{}/permissions", self.percent_encode(user)))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::Permissions>>()
            .map_err(Error::from)
    }

    pub fn get_permissions(&self, vhost: &str, user: &str) -> Result<responses::Permissions> {
        let response = self.http_get(&format!(
            "permissions/{}/{}",
            self.percent_encode(vhost),
            self.percent_encode(user)
        ))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<responses::Permissions>()
            .map_err(Error::from)
    }

    //
    // Rebalancing
    //

    pub fn rebalance_queue_leaders(&self) -> Result<()> {
        let m: HashMap<String, Value> = HashMap::new();
        self.http_post("rebalance/queues", &m)?;

        Ok(())
    }

    //
    // Health Checks
    //

    pub fn health_check_cluster_wide_alarms(&self) -> Result<()> {
        self.health_check_alarms("health/checks/alarms")
    }

    pub fn health_check_local_alarms(&self) -> Result<()> {
        self.health_check_alarms("health/checks/local-alarms")
    }

    pub fn health_check_if_node_is_quorum_critical(&self) -> Result<()> {
        let response = self.http_get("health/checks/node-is-quorum-critical")?;
        let response2 = self.ok_or_status_code_error_except_503(response)?;

        if response2.status().is_success() {
            return Ok(());
        }

        let failure_details = response2
            .json::<responses::QuorumCriticalityCheckDetails>()
            .map_err(Error::from)?;
        Err(Error::HealthCheckFailed(
            responses::HealthCheckFailureDetails::NodeIsQuorumCritical(failure_details),
        ))
    }

    //
    // Implementation
    //

    fn health_check_alarms(&self, path: &str) -> Result<()> {
        let response = self.http_get(path)?;
        let response2 = self.ok_or_status_code_error_except_503(response)?;

        if response2.status().is_success() {
            return Ok(());
        }

        let failure_details = response2
            .json::<responses::ClusterAlarmCheckDetails>()
            .map_err(Error::from)?;
        Err(Error::HealthCheckFailed(
            responses::HealthCheckFailureDetails::AlarmCheck(failure_details),
        ))
    }

    fn list_exchange_bindings_with_source_or_destination(
        &self,
        virtual_host: &str,
        exchange: &str,
        vertex: BindindVertex,
    ) -> Result<Vec<responses::BindingInfo>> {
        let response = self.http_get(&format!(
            "exchanges/{}/{}/bindings/{}",
            self.percent_encode(virtual_host),
            self.percent_encode(exchange),
            vertex
        ))?;
        let response2 = self.ok_or_status_code_error(response)?;
        response2
            .json::<Vec<responses::BindingInfo>>()
            .map_err(Error::from)
    }

    fn percent_encode(&self, value: &str) -> String {
        utf8_percent_encode(value, NON_ALPHANUMERIC).to_string()
    }

    fn http_get(&self, path: &str) -> crate::blocking::Result<HttpClientResponse> {
        let response = HttpClient::new()
            .get(self.rooted_path(path))
            .basic_auth(self.username, Some(self.password))
            .send();

        self.ok_or_http_client_error(response)
    }

    fn http_put<T>(&self, path: &str, payload: &T) -> crate::blocking::Result<HttpClientResponse>
    where
        T: Serialize,
    {
        let response = HttpClient::new()
            .put(self.rooted_path(path))
            .json(&payload)
            .basic_auth(self.username, Some(self.password))
            .send();

        self.ok_or_http_client_error(response)
    }

    fn http_post<T>(&self, path: &str, payload: &T) -> crate::blocking::Result<HttpClientResponse>
    where
        T: Serialize,
    {
        let response = HttpClient::new()
            .post(self.rooted_path(path))
            .json(&payload)
            .basic_auth(self.username, Some(self.password))
            .send();

        self.ok_or_http_client_error(response)
    }

    fn http_delete(&self, path: &str) -> crate::blocking::Result<HttpClientResponse> {
        let response = HttpClient::new()
            .delete(self.rooted_path(path))
            .basic_auth(self.username, Some(self.password))
            .send();
        self.ok_or_http_client_error(response)
    }

    fn ok_or_http_client_error(
        &self,
        result: reqwest::Result<HttpClientResponse>,
    ) -> Result<HttpClientResponse> {
        match result {
            Ok(val) => Ok(val),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn ok_or_status_code_error(&self, response: HttpClientResponse) -> Result<HttpClientResponse> {
        let status = response.status();
        if status.is_client_error() {
            return Err(Error::ClientErrorResponse(status.as_u16(), response));
        }

        if status.is_server_error() {
            return Err(Error::ServerErrorResponse(status.as_u16(), response));
        }

        Ok(response)
    }

    fn ok_or_status_code_error_except_503(
        &self,
        response: HttpClientResponse,
    ) -> Result<HttpClientResponse> {
        let status = response.status();
        if status.is_client_error() {
            return Err(Error::ClientErrorResponse(status.as_u16(), response));
        }

        // 503 Service Unavailable is used to indicate a health check failure.
        // In this case, we want to parse the response and provide a more specific error.
        if status.is_server_error() && status.as_u16() != 503 {
            return Err(Error::ServerErrorResponse(status.as_u16(), response));
        }

        Ok(response)
    }

    fn rooted_path(&self, path: &str) -> String {
        format!("{}/{}", self.endpoint, path)
    }
}

enum BindindVertex {
    Source,
    Destination,
}

impl Display for BindindVertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Source => write!(f, "source"),
            Self::Destination => write!(f, "destination"),
        }
    }
}
