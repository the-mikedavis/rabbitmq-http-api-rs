use serde::Serialize;

#[derive(Serialize)]
pub struct VirtualHostParams<'a> {
    pub name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_queue_type: Option<&'a str>,
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
