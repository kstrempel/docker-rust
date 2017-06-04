extern crate docker;

use docker::Client;
use docker::secrets::SecretsClient;

#[test]
fn get_all_images(){
    let client = Client::from_env();
    let secret_client = client.secrets();

    
}

#[test]
fn delete_unknown_id_images(){
    let client = Client::from_env();
    let secret_client = client.secrets();
    assert!(secret_client.delete(&String::from("alskjdaslkdj")).is_err());
}