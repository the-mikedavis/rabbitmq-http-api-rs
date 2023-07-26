use rabbitmq_http_client::blocking::Client;

mod common;
use crate::common::{endpoint, PASSWORD, USERNAME};
use serde_json::json;

#[test]
fn test_export_definitions() {
    let endpoint = endpoint();
    let rc = Client::new(&endpoint).with_basic_auth_credentials(USERNAME, PASSWORD);
    let result = rc.export_definitions();

    assert!(result.is_ok(), "export_definitions returned {:?}", result);
}

#[test]
fn test_import_definitions() {
    let endpoint = endpoint();
    let rc = Client::new(&endpoint).with_basic_auth_credentials(USERNAME, PASSWORD);
    let _ = rc.delete_queue("/", "imported_queue");
    let defs = json!({  "queues": [
      {
        "auto_delete": false,
        "durable": true,
        "name": "imported_queue",
        "vhost": "/"
      }
    ]});

    let result = rc.import_definitions(defs);
    assert!(result.is_ok(), "import_definitions returned {:?}", result);

    let result1 = rc.get_queue_info("/", "imported_queue");
    assert!(
        result1.is_ok(),
        "can't get the imported queue: {:?}",
        result1
    );
}
