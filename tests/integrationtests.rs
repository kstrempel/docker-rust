extern crate docker;

use docker::Client;
use docker::images::ImagesClient;
use docker::networks::NetworksClient;

#[test]
fn get_all(){
    let mut client = Client::from_env();

    assert!(client.images().all().unwrap().len()>1);
    assert!(client.networks().all().unwrap().len()>1);
}