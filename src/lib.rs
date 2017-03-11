#![crate_type = "lib"]
#![crate_name = "docker_rust"]

extern crate serde_json;
extern crate curl;

mod error;

use std::str::*;
use curl::easy::Easy;
use error::DockerError;

use std::ops::Add;
use std::error::Error;
use std::io::{stdout, Write};

pub struct Client {
    api_url: String,
    curl : Easy
}

impl Client {

    pub fn new(api_url: &str) -> Client {
        let mut client = Client {
            api_url : String::from_str(api_url).unwrap(),
            curl : Easy::new()
        };
        client.set_curl_client();
        client
    }

    pub fn from_env() -> Client {
        Client::new("unix:///var/run/docker.sock")
    }

    fn set_curl_client(&mut self) -> ()  {
        let _ = self.curl.unix_socket("/var/run/docker.sock");
    }

    fn url(& mut self, url: &str) -> String {
        let mut result = Vec::new();

        self.curl.url(url);
        let mut transfer = self.curl.transfer();
        transfer.write_function(|data| {
            result.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform();

        let mut json = String::new();
        for entry in result {
            json.push(entry as char)
        }

        json
    }

    pub fn images(& mut self) -> Result<(), DockerError> {
        let result = self.url("http:///v1.26/images/json");
        print!("{:?}", result);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use Client;
        let mut client = Client::from_env();
        let _ = client.images();

        assert!(false);
    }
}
