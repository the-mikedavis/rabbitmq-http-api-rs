use rabbitmq_http_client::{
    blocking::Client,
    commons::EnforcedLimitTarget,
    requests::{VirtualHostLimitParams, VirtualHostParams},
};

mod common;
use crate::common::{endpoint, PASSWORD, USERNAME};

#[test]
fn test_list_all_vhost_limits() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, PASSWORD);

    let vh_params = VirtualHostParams::named("test_list_all_vhost_limits");
    let result1 = rc.create_vhost(&vh_params);
    assert!(result1.is_ok());

    let limit = VirtualHostLimitParams::new(EnforcedLimitTarget::MaxQueues, 500);
    let result2 = rc.set_vhost_limit(&vh_params.name, limit);
    assert!(result2.is_ok());

    let result3 = rc.list_all_vhost_limits();
    assert!(result3.is_ok());
    let vec = result3.unwrap();
    assert!(vec.iter().find(|vh| vh.vhost == vh_params.name).is_some());

    let key1 = EnforcedLimitTarget::MaxConnections.to_string();
    assert!(vec
        .iter()
        .find(|it| it.vhost == vh_params.name && it.limits.get(&key1).is_some())
        .is_none());
    let key2 = EnforcedLimitTarget::MaxQueues.to_string();
    assert!(vec
        .iter()
        .find(|it| it.vhost == vh_params.name && it.limits.get(&key2).is_some())
        .is_some());

    rc.delete_vhost(vh_params.name).unwrap();
}

#[test]
fn test_list_vhost_limits() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, PASSWORD);

    let vh_params = VirtualHostParams::named("test_list_vhost_limits");
    let result1 = rc.create_vhost(&vh_params);
    assert!(result1.is_ok());

    let limit = VirtualHostLimitParams::new(EnforcedLimitTarget::MaxConnections, 500);
    let result2 = rc.set_vhost_limit(&vh_params.name, limit);
    assert!(result2.is_ok());

    let result3 = rc.list_vhost_limits(vh_params.name);
    assert!(result3.is_ok());
    let vec = result3.unwrap();

    let key1 = EnforcedLimitTarget::MaxConnections.to_string();
    assert!(vec
        .iter()
        .find(|it| it.vhost == vh_params.name && it.limits.get(&key1).is_some())
        .is_some());
    let key2 = EnforcedLimitTarget::MaxQueues.to_string();
    assert!(vec
        .iter()
        .find(|it| it.vhost == vh_params.name && it.limits.get(&key2).is_some())
        .is_none());

    rc.delete_vhost(vh_params.name).unwrap();
}
