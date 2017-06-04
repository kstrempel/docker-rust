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
pub mod schema;

use serde::{Deserialize, Serialize};
use serde_json;

use super::Client;
use super::error::DockerError;

use self::schema::Message;


pub struct HttpResult {
    pub response_code: u32,
    pub body : String,
}


macro_rules! endpoint {
    ($sty:ident) => (
        pub struct $sty<'a> {
            client : &'a Client
        }

        impl<'a> $sty<'a> {
            pub fn new(client : &Client) -> $sty {
                $sty {
                    client: client
                }
            }
        }
    )
}

// generic function to return a list from a json request 
pub fn get_vector<T: Deserialize> (client: &Client, path : &str) -> Result<Vec<T>, DockerError> {
    let result_raw = client.get(path).unwrap();
    let results : Vec<T> = serde_json::from_str(result_raw.as_str()).unwrap();

    Ok(results)    
}

// generic function to return a object from a json request
pub fn get<T: Deserialize> (client: &Client, path : &str) -> Result<T, DockerError> {
    let result_raw = client.get(path).unwrap();
    let result : T = serde_json::from_str(result_raw.as_str()).unwrap();

    Ok(result)    
}

// generic function to post a json object and retriews a json object
pub fn post<T: Serialize, R: Deserialize> (client: &Client, path : &str, payload : &T) -> Result<R, DockerError> {
    let payload_raw = serde_json::to_string(payload).unwrap();
    let result_raw = client.post(path, payload_raw.as_bytes()).unwrap();
    let result : R = serde_json::from_str(result_raw.as_str()).unwrap();

    Ok(result)
}

// generic function to call a delete api
pub fn delete(client: &Client, path : &str) -> Result<(), DockerError> {
    let response = client.delete(path)?;

    match response.response_code {
        204 => Ok(()),
        _ => {
            let message : Message = serde_json::from_str(&response.body)?;
            Err(DockerError::Docker(message.message))
        }
    }
}
