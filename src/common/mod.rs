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

use super::Client;
use super::error::DockerError;
use serde::Deserialize;
use serde_json;

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

pub fn get_vector<T : Deserialize> (client: &Client, path : &str) -> Result<Vec<T>, DockerError> {
    let result_raw = client.get(path).unwrap();
    let results : Vec<T> = serde_json::from_str(result_raw.as_str()).unwrap();

    Ok(results)    
}

pub fn get<T : Deserialize> (client: &Client, path : &str) -> Result<T, DockerError> {
    let result_raw = client.get(path).unwrap();
    let result : T = serde_json::from_str(result_raw.as_str()).unwrap();

    Ok(result)    
}


