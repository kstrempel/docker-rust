#![crate_type = "lib"]
#![crate_name = "docker"]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate curl;

pub mod images;
pub mod networks;
pub mod containers;
mod common;
mod error;

use std::str::*;
use std::cell::RefCell;

use curl::easy::Easy;

use error::DockerError;
use images::ImagesClient;
use containers::ContainersClient;
use networks::NetworksClient;

use std::error::Error;


pub struct Client {
    api_url: String,
    curl : RefCell<Easy>
}

impl Client {

    pub fn new(api_url: &str) -> Client {
        let client = Client {
            api_url : String::from_str(api_url).unwrap(),
            curl : RefCell::new(Easy::new())
        };
        client.set_curl_client();

        client
    }

    pub fn from_env() -> Client {
        Client::new("http:///v1.26/")
    }

    fn set_curl_client(&self) -> ()  {
        let mut curl = self.curl.borrow_mut();
        let _ = curl.unix_socket("/var/run/docker.sock");
    }

    fn get(&self, url: &str) -> Result<String, DockerError> {
        let mut result = Vec::new();
        let real_url = format!("{}{}", self.api_url, url);
        let mut curl = self.curl.borrow_mut();
        match curl.url(real_url.as_str()) {
            Ok(_) => {            
                let mut transfer = curl.transfer();
                transfer.write_function(|data| {
                    result.extend_from_slice(data);
                    Ok(data.len())
                }).unwrap();
                transfer.perform().unwrap();
            },
            Err(err) => {
                print!("Mein Text {}", err.description());
            }
        };
        
        Ok(String::from_utf8(result).unwrap())
    }   

    pub fn images(&self) -> ImagesClient {
        ImagesClient::new(self)
    } 
    pub fn containers(&self) -> ContainersClient {
        ContainersClient::new(self)
    } 

    pub fn networks(&self) -> NetworksClient {
        NetworksClient::new(self)
    } 
}
