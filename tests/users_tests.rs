use rabbitmq_http_client::blocking::Client;

mod common;
use crate::common::{endpoint, PASSWORD, USERNAME};

#[test]
fn test_list_users() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, Some(PASSWORD));
    let result = rc.list_users();

    assert!(result.is_ok());
    let vec = result.unwrap();
    assert!(vec.iter().find(|u| u.name == "guest").is_some())
}

#[test]
fn test_get_user() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, Some(PASSWORD));
    let name = "guest";
    let result = rc.get_user(name);

    assert!(result.is_ok());
    let u = result.unwrap();
    assert!(u.name == name);
}
