extern crate docker;

use docker::Client;

#[test]
fn get_all_images(){
    let client = Client::from_env();
    assert!(client.images().all().is_ok())
}

#[test]
fn get_all_containers(){
    let client = Client::from_env();
    assert!(client.containers().all().is_ok())
}

#[test]
fn get_all_networks(){
    let client = Client::from_env();
    assert!(client.networks().all().is_ok())
}

#[test]
fn get_swarm(){
    let client = Client::from_env();
    assert!(client.swarm().get().is_ok());
}

#[test]
fn get_volums(){
    let client = Client::from_env();
    assert!(client.volumes().get().is_ok())
}

#[test]
fn get_tasks(){
    let client = Client::from_env();
    assert!(client.tasks().get().is_ok())
}

#[test]
fn get_all_secrets(){
    let client = Client::from_env();
    assert!(client.secrets().all().is_ok())
}

