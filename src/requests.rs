use serde::Serialize;

#[derive(Serialize)]
pub struct VirtualHostParams<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub tags: Vec<&'a str>,
    pub default_queue_type: &'a str,
    pub tracing: bool
}