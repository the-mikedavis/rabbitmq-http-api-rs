use rabbitmq_http_client::blocking::Client;
use rabbitmq_http_client::requests::VirtualHostParams;
use rabbitmq_http_client::responses;

mod common;
use crate::common::{endpoint, PASSWORD, USERNAME};

#[test]
fn test_list_permissions() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, PASSWORD);

    let vh_params = VirtualHostParams {
        name: "test_list_permissions",
        tracing: false,
        description: None,
        tags: None,
        default_queue_type: None,
    };
    let _ = rc.delete_vhost(vh_params.name);
    let result1 = rc.create_vhost(&vh_params);
    assert!(result1.is_ok());

    let result = rc.list_permissions();
    assert!(result.is_ok());

    let vec = result.unwrap();
    assert!(vec.iter().any(|p| p
        == &responses::Permissions {
            user: "guest".to_owned(),
            vhost: "test_list_permissions".to_owned(),
            configure: ".*".to_owned(),
            read: ".*".to_owned(),
            write: ".*".to_owned(),
        }));

    let _ = rc.delete_vhost(vh_params.name);
}

#[test]
fn test_list_permissions_in() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, PASSWORD);

    let vh_params = VirtualHostParams {
        name: "test_list_permissions_in",
        tracing: false,
        description: None,
        tags: None,
        default_queue_type: None,
    };
    let _ = rc.delete_vhost(vh_params.name);
    let result1 = rc.create_vhost(&vh_params);
    assert!(result1.is_ok());

    let result = rc.list_permissions_in("test_list_permissions_in");
    assert!(result.is_ok(), "list_permissions_in returned {:?}", result);

    let vec = result.unwrap();
    assert!(vec.iter().any(|p| p
        == &responses::Permissions {
            user: "guest".to_owned(),
            vhost: "test_list_permissions_in".to_owned(),
            configure: ".*".to_owned(),
            read: ".*".to_owned(),
            write: ".*".to_owned(),
        }));

    let _ = rc.delete_vhost(vh_params.name);
}

#[test]
fn test_list_permissions_of() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, PASSWORD);

    let vh_params = VirtualHostParams {
        name: "test_list_permissions_of",
        tracing: false,
        description: None,
        tags: None,
        default_queue_type: None,
    };
    let _ = rc.delete_vhost(vh_params.name);
    let result1 = rc.create_vhost(&vh_params);
    assert!(result1.is_ok());

    let result = rc.list_permissions_of("guest");
    assert!(result.is_ok(), "list_permissions_of returned {:?}", result);

    let vec = result.unwrap();
    assert!(vec.iter().any(|p| p
        == &responses::Permissions {
            user: "guest".to_owned(),
            vhost: "test_list_permissions_of".to_owned(),
            configure: ".*".to_owned(),
            read: ".*".to_owned(),
            write: ".*".to_owned(),
        }));

    let _ = rc.delete_vhost(vh_params.name);
}

#[test]
fn test_get_permissions() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, PASSWORD);

    let vh_params = VirtualHostParams {
        name: "test_get_permissions",
        tracing: false,
        description: None,
        tags: None,
        default_queue_type: None,
    };
    let _ = rc.delete_vhost(vh_params.name);
    let result1 = rc.create_vhost(&vh_params);
    assert!(result1.is_ok());

    let result2 = rc.get_permissions("test_get_permissions", "guest");
    assert!(
        result2.is_ok(),
        "list_permissions_of returned {:?}",
        result2
    );

    let result3 = result2.unwrap();
    assert_eq!(
        result3,
        responses::Permissions {
            user: "guest".to_owned(),
            vhost: "test_get_permissions".to_owned(),
            configure: ".*".to_owned(),
            read: ".*".to_owned(),
            write: ".*".to_owned(),
        }
    );

    let _ = rc.delete_vhost(vh_params.name);
}
