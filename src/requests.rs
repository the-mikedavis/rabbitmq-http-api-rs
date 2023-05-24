use serde::Serialize;

#[derive(Serialize)]
pub struct VirtualHostParams<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub tags: Option<Vec<&'a str>>,
    pub default_queue_type: Option<&'a str>,
    pub tracing: bool
}