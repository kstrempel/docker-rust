extern crate docker;

use docker::Client;

#[test]
fn get_all(){
    let client = Client::from_env();

    assert!(client.images().all().unwrap().len()>1);
    assert!(client.containers().all().unwrap().len()>1);
    assert!(client.networks().all().unwrap().len()>1);
    assert!(client.swarm().get().is_ok());
    assert!(client.volumes().get().is_ok());
    assert!(client.tasks().get().is_ok());
}