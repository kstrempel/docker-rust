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
pub struct Port {
    #[serde(rename = "PrivatePort")]
    pub private_port: u64,
    #[serde(rename = "PublicPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_port: Option<u64>,
    #[serde(rename = "Type")]
    pub port_type: String
}
    
#[derive(Serialize, Deserialize, Debug)]
pub struct Bridge {
    #[serde(rename = "NetworkID")]
    pub network_id : String,
    #[serde(rename = "EndpointID")]
    pub endpoint_id : String,
    #[serde(rename = "Gateway")]
    pub gateway : String,
    #[serde(rename = "IPAddress")]
    pub ip_address : String,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: u64,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway : String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address : String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_preflix_len:  u64,
    #[serde(rename = "MacAddress")]
    pub mac_address : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mount {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Source")]
    pub source : String,
    #[serde(rename = "Destination")]
    pub destination: String,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "Mode")]
    pub mode : String, 
    #[serde(rename = "RW")]
    pub rw : bool,
    #[serde(rename = "Propagation")]
    pub propagation : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Names")]
    pub names: Vec<String>,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "ImageID")]
    pub image_id: String,
    #[serde(rename = "Command")]
    pub command: String,
    #[serde(rename = "Created")]
    pub created: u64,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Ports")]
    pub ports: Vec<Port>,
    #[serde(rename = "Labels")]
    pub labels: HashMap<String, String>,
    #[serde(rename = "SizeRw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_rw: Option<u64>,
    #[serde(rename = "SizeRootFs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_root_fs: Option<u64>,
    #[serde(rename = "HostConfig")]
    pub host_config: HashMap<String,String>,
    #[serde(rename = "NetworkSettings")]
    pub network_settings: HashMap<String,HashMap<String,Bridge>>,
    #[serde(rename = "Mounts")]
    pub mounts: Vec<Mount>
}