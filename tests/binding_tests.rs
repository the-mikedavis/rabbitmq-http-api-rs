use rabbitmq_http_client::{blocking::Client, requests::QueueParams};

mod common;
use crate::common::{endpoint, PASSWORD, USERNAME};

#[test]
fn test_list_bindings() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, Some(PASSWORD));

    let vh_name = "/";
    let cq = "rust.cq.durable.1";
    let fanout = "amq.fanout";

    let result1 = rc.declare_queue(vh_name, &QueueParams::new_durable_classic_queue(cq, None));
    assert!(result1.is_ok());
    
    let result2 = rc.bind_queue(vh_name, cq, fanout, None, None);
    assert!(result2.is_ok());

    let result3 = rc.list_bindings();
    assert!(result3.is_ok());
    let vec = result3.unwrap();
    assert!(vec.iter().find(|vh| vh.destination == cq && vh.source == fanout).is_some());
    
    let result4 = rc.list_bindings_in(vh_name);
    assert!(result4.is_ok());
    let vec = result4.unwrap();
    assert!(vec.iter().find(|vh| vh.vhost == vh_name && vh.source == fanout).is_some());    
    
    let _ = rc.delete_queue(vh_name, cq);
}
