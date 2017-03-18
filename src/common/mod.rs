use super::Client;
use super::error::DockerError;
use serde::Deserialize;
use serde_json;

pub fn get<T : Deserialize> (client: &Client, path : &str) -> Result<Vec<T>, DockerError> {
    let result = client.get(path).unwrap();
    let images : Vec<T> = serde_json::from_str(result.as_str()).unwrap();

    Ok(images)    
}