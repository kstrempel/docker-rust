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

pub fn get<T : Deserialize> (client: &Client, path : &str) -> Result<Vec<T>, DockerError> {
    let result = client.get(path).unwrap();
    let images : Vec<T> = serde_json::from_str(result.as_str()).unwrap();

    Ok(images)    
}