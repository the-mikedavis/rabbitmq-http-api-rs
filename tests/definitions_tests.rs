use rabbitmq_http_client::blocking::Client;

mod common;
use crate::common::{endpoint, PASSWORD, USERNAME};

#[test]
fn test_export_definitions() {
    let endpoint = endpoint();
    let rc = Client::new(&endpoint).with_basic_auth_credentials(USERNAME, PASSWORD);
    let result = rc.export_definitions();

    assert!(result.is_ok(), "export_definitions returned {:?}", result);
}
