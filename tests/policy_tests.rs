use rabbitmq_http_client::{blocking::Client, requests::PolicyParams};

use serde_json::{json, Map, Value};
mod common;
use crate::common::{endpoint, PASSWORD, USERNAME};

#[test]
fn test_message_ttl_policy() {
    let mut map = Map::<String, Value>::new();
    map.insert("message-ttl".to_owned(), json!(10_000));
    let policy_definition = Some(map);

    let message_ttl_policy = PolicyParams {
        vhost: "/",
        name: "message_ttl_policy",
        pattern: ".*",
        apply_to: "all",
        priority: 42,
        definition: policy_definition,
    };
    test_a_policy(&message_ttl_policy)
}

#[test]
fn test_dlx_policy() {
    let mut map = Map::<String, Value>::new();
    map.insert("dead-letter-exchange".to_owned(), json!("my-dlx"));
    let policy_definition = Some(map);

    let dlx_policy = PolicyParams {
        vhost: "/",
        name: "dlx_policy",
        pattern: ".*",
        apply_to: "quorum_queues",
        priority: 0,
        definition: policy_definition,
    };
    test_a_policy(&dlx_policy)
}

fn test_a_policy(policy: &PolicyParams) {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, Some(PASSWORD));

    // initially, there should be no such policy
    let policies = rc.list_policies().unwrap();
    assert!(!policies
        .iter()
        .find(|policy| policy.name == policy.name)
        .is_some());

    let result = rc.declare_policy(&policy);
    assert!(result.is_ok());

    // validate it was created as expected
    let result = rc.get_policy("/", policy.name);
    assert!(result.unwrap().definition == policy.definition);

    // delete it
    assert!(rc.delete_policy("/", policy.name).is_ok());

    // there should be no such policy anymore
    let policies = rc.list_policies().unwrap();
    assert!(!policies.iter().find(|p| p.name == policy.name).is_some());
}
