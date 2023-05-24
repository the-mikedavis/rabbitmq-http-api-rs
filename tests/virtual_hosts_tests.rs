use rabbitmq_http_client::{blocking::Client, requests::VirtualHostParams};

mod common;
use crate::common::{endpoint, USERNAME, PASSWORD};

#[test]
fn test_list_vhosts() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, Some(PASSWORD));
    let result = rc.list_vhosts();
    
    assert!(result.is_ok());
    let vec = result.unwrap();
    assert!(vec.iter().find(|vh| vh.name == "/").is_some())
}

#[test]
fn test_get_vhost() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, Some(PASSWORD));
    let name = "/";
    let result = rc.get_vhost(name);

    assert!(result.is_ok());
    let vh = result.unwrap();
    assert!(vh.name == name);
}

#[test]
fn test_create_vhost() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, Some(PASSWORD));
    let name = "rust_test_create_vhost";

    let _ = rc.delete_vhost(&name);

    let result1 = rc.get_vhost(name);
    assert!(!result1.is_ok());

    let params = VirtualHostParams {
        name,
        description: &format!("{} description", &name),
        tags: vec!["tag-a", "tag-b"],
        default_queue_type: "classic",
        tracing: false,
    };
    let result2 = rc.create_vhost(&params);
    assert!(result2.is_ok());

    let result3 = rc.get_vhost(name);
    assert!(result3.is_ok());
    let vh2 = result3.unwrap();
    assert!(vh2.name == name);
}

#[test]
fn test_delete_vhost() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, Some(PASSWORD));
    let name = "rust_test_delete_vhost";

    let params = VirtualHostParams {
        name,
        description: &format!("{} description", &name),
        tags: vec!["tag-a", "tag-b"],
        default_queue_type: "classic",
        tracing: false,
    };
    let result1 = rc.create_vhost(&params);
    assert!(result1.is_ok());

    let result2 = rc.get_vhost(name);
    assert!(result2.is_ok());

    let _ = rc.delete_vhost(&name);
    let result3 = rc.get_vhost(name);
    assert!(!result3.is_ok());
}