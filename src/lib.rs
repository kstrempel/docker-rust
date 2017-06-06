// Copyright 2017 Kai Strempel
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![crate_type = "lib"]
#![crate_name = "docker"]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate curl;

#[macro_use]
mod common;
mod error;

pub mod images;
pub mod networks;
pub mod containers;
pub mod swarm;
pub mod volumes;
pub mod tasks;
pub mod secrets;

use std::str::*;
use std::cell::RefCell;
use std::io::Read;

use curl::easy::Easy;

use error::DockerError;
use common::HttpResult;
use images::ImagesClient;
use containers::ContainersClient;
use networks::NetworksClient;
use swarm::SwarmClient;
use volumes::VolumesClient;
use tasks::TasksClient;
use secrets::SecretsClient;


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
        Client::new("http://v1.27/")
    }

    fn set_curl_client(&self) -> ()  {
        let mut curl = self.curl.borrow_mut();
        let _ = curl.unix_socket("/var/run/docker.sock");
    }

    fn get(&self, url: &str) -> Result<HttpResult, DockerError> {
        let mut result = Vec::new();
        let real_url = format!("{}{}", self.api_url, url);
        let mut curl = self.curl.borrow_mut();
        curl.get(true).unwrap();        
        curl.url(real_url.as_str()).unwrap();

        {
            let mut transfer = curl.transfer();
            transfer.write_function(|data| {
                result.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap()
        }

        Ok(HttpResult{response_code: curl.response_code().unwrap(),
                      body: String::from_utf8(result).unwrap()})
    }   

    fn post(&self, url: &str, mut payload: &[u8]) -> Result<HttpResult, DockerError> {
        let mut result = Vec::new();
        let mut curl = self.curl.borrow_mut();
        let real_url = format!("{}{}", self.api_url, url);
        curl.url(real_url.as_str()).unwrap();
        curl.post(true).unwrap();
        curl.post_field_size(payload.len() as u64).unwrap();

        {
            let mut transfer = curl.transfer();
            transfer.read_function(|buf| {
                Ok(payload.read(buf).unwrap_or(0))
            }).unwrap();

            transfer.write_function(|data| {
                result.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        Ok(HttpResult{response_code: curl.response_code().unwrap(),
                      body: String::from_utf8(result).unwrap()})
    }


    fn delete(&self, url: &str) -> Result<HttpResult, DockerError> {
        let mut result = Vec::new();
        let real_url = format!("{}{}", self.api_url, url);
        let mut curl = self.curl.borrow_mut();
        curl.custom_request("DELETE").unwrap();        
        curl.url(real_url.as_str()).unwrap();

        {
            let mut transfer = curl.transfer();
            transfer.write_function(|data| {
                result.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        Ok(HttpResult{response_code: curl.response_code().unwrap(),
                      body: String::from_utf8(result).unwrap()})
    }   

    pub fn images(&self) -> ImagesClient {
        ImagesClient::new(self)
    } 
    pub fn containers(&self) -> ContainersClient {
        ContainersClient::new(self)
    } 

    pub fn swarm(&self) -> SwarmClient {
        SwarmClient::new(self)
    } 

    pub fn networks(&self) -> NetworksClient {
        NetworksClient::new(self)
    } 

    pub fn volumes(&self) -> VolumesClient {
        VolumesClient::new(self)
    } 

    pub fn tasks(&self) -> TasksClient {
        TasksClient::new(self)
    }

    pub fn secrets(&self) -> SecretsClient {
        SecretsClient::new(self)
    }
}
