extern crate docker;

use std::collections::HashMap;

use docker::Client;
use docker::secrets::schema::SecretSpec;

#[test]
fn delete_unknown_id_images(){
    let client = Client::from_env();
    let secret_client = client.secrets();
    assert!(secret_client.delete(&String::from("alskjdaslkdj")).is_err());
}

#[test]
fn try_to_create_non_working_secret(){
    let client = Client::from_env();
    let secret_client = client.secrets();
    let secret = SecretSpec{
                        name: Some(String::from("NonWorkingDockerRustMySecret")),
                        labels: Option::Some(HashMap::new()),
                        data: Some(String::from("ThatWillNotWork"))
                     };  
    assert!(secret_client.create(&secret).is_err());    
}

#[test]
fn try_to_inpsect_unkown_secret(){
    let client = Client::from_env();
    let secret_client = client.secrets();
    assert!(secret_client.inspect(&String::from("unknown_id")).is_err());
}


#[test]
fn try_to_update_unkown_secret(){
    let client = Client::from_env();
    let secret_client = client.secrets();
    let secret = SecretSpec{
                        name: Some(String::from("NonWorkingDockerRustMySecret")),
                        labels: Option::Some(HashMap::new()),
                        data: Some(String::from("ThatWillNotWork"))
                     };  
    assert!(secret_client.inspect(&String::from("unknown_id")).is_err());
}