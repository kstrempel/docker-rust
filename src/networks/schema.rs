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


use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct IPAM {
    #[serde(rename = "Driver")]
    pub driver: String,
    #[serde(rename = "Options")]
    pub options: String,
    #[serde(rename = "Config")]
    pub config: Vec<HashMap<String,String>>,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "Scope")]
    pub scope: String,
    #[serde(rename = "Driver")]
    pub driver: String,
    #[serde(rename = "EnableIPv6")]
    pub enable_ip_v6: bool,
    #[serde(rename = "Internal")]
    pub internal: bool,
    #[serde(rename = "Attachable")]
    pub attachable: bool,
    #[serde(rename = "IPAM")]
    pub ipam : IPAM,
    #[serde(rename = "Containers")]
    pub containers : HashMap<String, HashMap<String,String>>,
    #[serde(rename = "Options")]
    pub options : HashMap<String,String>,
    #[serde(rename = "Labels")]
    pub labels: HashMap<String, String>,    
}