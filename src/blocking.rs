use crate::{
    requests::{
        ExchangeParams, QueueParams, RuntimeParameterDefinition, UserParams, VirtualHostParams,
    },
    responses,
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::blocking::Client as HttpClient;
use serde::Serialize;
use std::collections::HashMap;

pub struct Client<'a> {
    endpoint: &'a str,
    username: &'a str,
    password: Option<&'a str>,
}

impl<'a> Client<'a> {
    pub fn new_with_basic_auth_credentials(
        endpoint: &'a str,
        username: &'a str,
        password: Option<&'a str>,
    ) -> Self {
        Self {
            endpoint: endpoint,
            username: username,
            password: password,
        }
    }

    pub fn list_nodes(&self) -> responses::Result<Vec<responses::ClusterNode>> {
        let response = self.http_get("nodes")?;
        response.json::<Vec<responses::ClusterNode>>()
    }

    pub fn list_vhosts(&self) -> responses::Result<Vec<responses::VirtualHost>> {
        let response = self.http_get("vhosts")?;
        response.json::<Vec<responses::VirtualHost>>()
    }

    pub fn list_users(&self) -> responses::Result<Vec<responses::User>> {
        let response = self.http_get("users")?;
        response.json::<Vec<responses::User>>()
    }

    pub fn list_connections(&self) -> responses::Result<Vec<responses::Connection>> {
        let response = self.http_get("connections")?;
        response.json::<Vec<responses::Connection>>()
    }

    pub fn list_channels(&self) -> responses::Result<Vec<responses::Channel>> {
        let response = self.http_get("channels")?;
        response.json::<Vec<responses::Channel>>()
    }

    pub fn list_queues(&self) -> responses::Result<Vec<responses::QueueInfo>> {
        let response = self.http_get("queues")?;
        response.json::<Vec<responses::QueueInfo>>()
    }

    pub fn list_queues_in(
        &self,
        virtual_host: &str,
    ) -> responses::Result<Vec<responses::QueueInfo>> {
        let response = self.http_get(&format!("queues/{}", self.percent_encode(virtual_host)))?;
        response.json::<Vec<responses::QueueInfo>>()
    }

    pub fn list_exchanges(&self) -> responses::Result<Vec<responses::ExchangeInfo>> {
        let response = self.http_get("exchanges")?;
        response.json::<Vec<responses::ExchangeInfo>>()
    }

    pub fn list_exchanges_in(
        &self,
        virtual_host: &str,
    ) -> responses::Result<Vec<responses::ExchangeInfo>> {
        let response =
            self.http_get(&format!("exchanges/{}", self.percent_encode(virtual_host)))?;
        response.json::<Vec<responses::ExchangeInfo>>()
    }

    pub fn list_consumers(&self) -> responses::Result<Vec<responses::Consumer>> {
        let response = self.http_get("consumers")?;
        response.json::<Vec<responses::Consumer>>()
    }

    pub fn get_node_info(&self, name: &str) -> responses::Result<responses::ClusterNode> {
        let response = self.http_get(&format!("nodes/{}", name))?;
        let node = response.json::<responses::ClusterNode>()?;
        Ok(node)
    }

    pub fn get_vhost(&self, name: &str) -> responses::Result<responses::VirtualHost> {
        let response = self.http_get(&format!("vhosts/{}", self.percent_encode(name)))?;
        let node = response.json::<responses::VirtualHost>()?;
        Ok(node)
    }

    pub fn get_user(&self, name: &str) -> responses::Result<responses::User> {
        let response = self.http_get(&format!("users/{}", self.percent_encode(name)))?;
        let node = response.json::<responses::User>()?;
        Ok(node)
    }

    pub fn get_queue_info(
        &self,
        virtual_host: &str,
        name: &str,
    ) -> responses::Result<responses::QueueInfo> {
        let response = self.http_get(&format!(
            "queues/{}/{}",
            self.percent_encode(&virtual_host),
            self.percent_encode(&name)
        ))?;
        let queue = response.json::<responses::QueueInfo>()?;
        Ok(queue)
    }

    pub fn get_exchange_info(
        &self,
        virtual_host: &str,
        name: &str,
    ) -> responses::Result<responses::ExchangeInfo> {
        let response = self.http_get(&format!(
            "exchanges/{}/{}",
            self.percent_encode(&virtual_host),
            self.percent_encode(&name)
        ))?;
        let exchange = response.json::<responses::ExchangeInfo>()?;
        Ok(exchange)
    }

    pub fn create_vhost(&self, params: &VirtualHostParams) -> responses::Result<()> {
        self.update_vhost(params)
    }

    pub fn update_vhost(&self, params: &VirtualHostParams) -> responses::Result<()> {
        let _ = self.http_put(
            &format!("vhosts/{}", self.percent_encode(&params.name)),
            params,
        )?;
        Ok(())
    }

    pub fn create_user(&self, params: &UserParams) -> responses::Result<()> {
        let _ = self.http_put(
            &format!("users/{}", self.percent_encode(&params.name)),
            params,
        )?;
        Ok(())
    }

    pub fn declare_queue(&self, virtual_host: &str, params: &QueueParams) -> responses::Result<()> {
        let _ = self.http_put(
            &format!(
                "queues/{}/{}",
                self.percent_encode(&virtual_host),
                self.percent_encode(&params.name)
            ),
            params,
        )?;
        Ok(())
    }

    pub fn declare_exchange(
        &self,
        virtual_host: &str,
        params: &ExchangeParams,
    ) -> responses::Result<()> {
        let _ = self.http_put(
            &format!(
                "exchanges/{}/{}",
                self.percent_encode(&virtual_host),
                self.percent_encode(&params.name)
            ),
            params,
        )?;
        Ok(())
    }

    pub fn delete_vhost(&self, virtual_host: &str) -> responses::Result<()> {
        self.http_delete(&format!("vhosts/{}", self.percent_encode(virtual_host)))?;
        Ok(())
    }

    pub fn delete_user(&self, username: &str) -> responses::Result<()> {
        self.http_delete(&format!("users/{}", self.percent_encode(username)))?;
        Ok(())
    }

    pub fn delete_queue(&self, virtual_host: &str, name: &str) -> responses::Result<()> {
        self.http_delete(&format!(
            "queues/{}/{}",
            self.percent_encode(virtual_host),
            self.percent_encode(name)
        ))?;
        Ok(())
    }

    pub fn delete_exchange(&self, virtual_host: &str, name: &str) -> responses::Result<()> {
        self.http_delete(&format!(
            "exchanges/{}/{}",
            self.percent_encode(virtual_host),
            self.percent_encode(name)
        ))?;
        Ok(())
    }

    pub fn purge_queue(&self, virtual_host: &str, name: &str) -> responses::Result<()> {
        self.http_delete(&format!(
            "queues/{}/{}/contents",
            self.percent_encode(virtual_host),
            self.percent_encode(name)
        ))?;
        Ok(())
    }

    pub fn list_runtime_parameters(&self) -> responses::Result<Vec<responses::RuntimeParameter>> {
        let response = self.http_get("parameters")?;
        response.json::<Vec<responses::RuntimeParameter>>()
    }

    pub fn list_runtime_parameters_of_component(
        &self,
        component: &str,
    ) -> responses::Result<Vec<responses::RuntimeParameter>> {
        let path = format!("parameters/{}", self.percent_encode(component));
        let response = self.http_get(&path)?;
        response.json::<Vec<responses::RuntimeParameter>>()
    }

    pub fn list_runtime_parameters_of_component_in(
        &self,
        component: &str,
        vhost: &str,
    ) -> responses::Result<Vec<responses::RuntimeParameter>> {
        let path = format!(
            "parameters/{}/{}",
            self.percent_encode(component),
            self.percent_encode(vhost)
        );
        let response = self.http_get(&path)?;
        response.json::<Vec<responses::RuntimeParameter>>()
    }

    pub fn get_runtime_parameter(
        &self,
        component: &str,
        vhost: &str,
        name: &str,
    ) -> responses::Result<responses::RuntimeParameter> {
        let path = format!(
            "parameters/{}/{}/{}",
            self.percent_encode(component),
            self.percent_encode(vhost),
            self.percent_encode(name)
        );
        let response = self.http_get(&path)?;
        response.json::<responses::RuntimeParameter>()
    }

    pub fn upsert_runtime_parameter(
        &self,
        param: &RuntimeParameterDefinition,
    ) -> responses::Result<()> {
        let path = format!(
            "parameters/{}/{}/{}",
            self.percent_encode(&param.component),
            self.percent_encode(&param.vhost),
            self.percent_encode(&param.name)
        );
        self.http_put(&path, &param)?;
        Ok(())
    }

    pub fn clear_runtime_parameter(
        &self,
        component: &str,
        vhost: &str,
        name: &str,
    ) -> responses::Result<()> {
        let path = format!(
            "parameters/{}/{}/{}",
            self.percent_encode(&component),
            self.percent_encode(&vhost),
            self.percent_encode(&name)
        );
        self.http_delete(&path)?;
        Ok(())
    }

    pub fn clear_all_runtime_parameters(&self) -> responses::Result<()> {
        let params = self.list_runtime_parameters()?;
        for rp in params {
            self.clear_runtime_parameter(&rp.component, &rp.vhost, &rp.name)?
        }
        Ok(())
    }

    pub fn get_cluster_name(&self) -> responses::Result<responses::ClusterIdentity> {
        let response = self.http_get("cluster-name")?;
        let id = response.json::<responses::ClusterIdentity>()?;
        Ok(id)
    }

    pub fn set_cluster_name(&self, new_name: &str) -> responses::Result<()> {
        let mut map = HashMap::new();
        map.insert("name", new_name);

        let _ = self.http_put("cluster-name", &map)?;
        Ok(())
    }

    //
    // Implementation
    //

    fn percent_encode(&self, value: &str) -> String {
        utf8_percent_encode(value, NON_ALPHANUMERIC).to_string()
    }

    fn http_get(&self, path: &str) -> reqwest::Result<reqwest::blocking::Response> {
        HttpClient::new()
            .get(self.rooted_path(path))
            .basic_auth(self.username, self.password)
            .send()
    }

    fn http_put<T>(&self, path: &str, payload: &T) -> reqwest::Result<reqwest::blocking::Response>
    where
        T: Serialize,
    {
        HttpClient::new()
            .put(self.rooted_path(path))
            .json(&payload)
            .basic_auth(self.username, self.password)
            .send()
    }

    fn http_delete(&self, path: &str) -> reqwest::Result<reqwest::blocking::Response> {
        HttpClient::new()
            .delete(self.rooted_path(path))
            .basic_auth(self.username, self.password)
            .send()
    }

    fn rooted_path(&self, path: &str) -> String {
        format!("{}/{}", self.endpoint, path)
    }
}
