use rabbitmq_http_client::blocking::Client;
use rabbitmq_http_client::requests::RuntimeParameterDefinition;
use serde_json::{json, Map, Value};

mod common;
use crate::common::{await_metric_emission, endpoint, PASSWORD, USERNAME};

#[test]
fn test_upsert_runtime_parameter() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, Some(PASSWORD));

    let mut val = max_connections_limit(9988);
    let rpf = example_runtime_parameter_definition(&mut val);
    let result1 = rc.upsert_runtime_parameter(&rpf);
    assert!(result1.is_ok());

    let result2 = rc.get_runtime_parameter(&rpf.component, &rpf.vhost, &rpf.name);
    assert!(result2.is_ok());
    assert_eq!(9988, result2.unwrap().value.get("max-connections").unwrap().as_u64().unwrap());

    let _ = rc.clear_runtime_parameter(&rpf.component, &rpf.vhost, &rpf.name);
}

#[test]
fn test_clear_runtime_parameter() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, Some(PASSWORD));

    let mut val = max_queue_limit(4444);
    let rp = example_runtime_parameter_definition(&mut val);
    let result1 = rc.upsert_runtime_parameter(&rp);
    assert!(result1.is_ok());
    await_metric_emission(1000);

    let result2 = rc.clear_runtime_parameter("vhost-limits", "/", "limits");
    assert!(result2.is_ok());

    let result3 = rc.list_runtime_parameters();
    assert!(result3.is_ok());
    let vec = result3.unwrap();
    assert!(vec.is_empty());
}

//
// Implementation
//

fn max_connections_limit(n: usize) -> Map<String, Value> {
    let mut val = Map::<String, Value>::new();
    val.insert(String::from("max-connections"), json!(n));
    val
}

fn max_queue_limit(n: usize) -> Map<String, Value> {
    let mut val = Map::<String, Value>::new();
    val.insert(String::from("max-queues"), json!(n));
    val
}

fn example_runtime_parameter_definition(val: &mut Map<String, Value>) -> RuntimeParameterDefinition {
    RuntimeParameterDefinition {
        vhost: "/".to_owned(),
        name: "limits".to_owned(),
        component: "vhost-limits".to_owned(),
        value: val.clone(),
    }
}