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
   // Name of the IPAM driver to use.
   #[serde(rename = "Driver")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub driver : Option<String>,

   // List of IPAM configuration options, specified as a map: `{\"Subnet\": <CIDR>, \"IPRange\": <CIDR>, \"Gateway\": <IP address>, \"AuxAddress\": <device_name:IP address>}`
   #[serde(rename = "Config")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub config : Option<Vec<HashMap<String,String>>>,

   // Driver-specific options, specified as a map.
   #[serde(rename = "Options")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub options : Option<Vec<HashMap<String,String>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkContainer {

   #[serde(rename = "EndpointID")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub endpoint_id : Option<String>,

   #[serde(rename = "MacAddress")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub mac_address : Option<String>,

   #[serde(rename = "IPv4Address")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub i_pv4_address : Option<String>,

   #[serde(rename = "IPv6Address")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub i_pv6_address : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
   #[serde(rename = "Name")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub name : Option<String>,

   #[serde(rename = "Id")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub id : Option<String>,

   #[serde(rename = "Created")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub created : Option<String>,

   #[serde(rename = "Scope")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub scope : Option<String>,

   #[serde(rename = "Driver")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub driver : Option<String>,

   #[serde(rename = "EnableIPv6")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub enable_i_pv6 : Option<bool>,

   #[serde(rename = "IPAM")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub ipam : Option<IPAM>,

   #[serde(rename = "Internal")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub internal : Option<bool>,

   #[serde(rename = "Attachable")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub attachable : Option<bool>,

   #[serde(rename = "Containers")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub containers : Option<HashMap<String,NetworkContainer>>,

   #[serde(rename = "Options")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub options : Option<HashMap<String,String>>,

   #[serde(rename = "Labels")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub labels : Option<HashMap<String,String>>,
}
