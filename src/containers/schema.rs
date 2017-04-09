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
pub struct HostConfig {
   #[serde(rename = "NetworkMode")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub network_mode : Option<String>
}


// IPAM configurations for the endpoint
#[derive(Serialize, Deserialize, Debug)]
pub struct EndpointSettingsIpamConfig {
   #[serde(rename = "IPv4Address")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub i_pv4_address : Option<String>,

   #[serde(rename = "IPv6Address")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub i_pv6_address : Option<String>,

   #[serde(rename = "LinkLocalIPs")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub link_local_i_ps : Option<Vec<String>>,
}


// Configuration for a network endpoint.
#[derive(Serialize, Deserialize, Debug)]
pub struct EndpointSettings {
   #[serde(rename = "IPAMConfig")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub ipam_config : Option<EndpointSettingsIpamConfig>,

   #[serde(rename = "Links")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub links : Option<Vec<String>>,

   #[serde(rename = "Aliases")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub aliases : Option<Vec<String>>,

   #[serde(rename = "NetworkID")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub network_id : Option<String>,

   #[serde(rename = "EndpointID")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub endpoint_id : Option<String>,

   #[serde(rename = "Gateway")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub gateway : Option<String>,

   #[serde(rename = "IPAddress")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub ip_address : Option<String>,

   #[serde(rename = "IPPrefixLen")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub ip_prefix_len : Option<i32>,

   #[serde(rename = "IPv6Gateway")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub i_pv6_gateway : Option<String>,

   #[serde(rename = "GlobalIPv6Address")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub global_i_pv6_address : Option<String>,

   #[serde(rename = "GlobalIPv6PrefixLen")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub global_i_pv6_prefix_len : Option<i64>,

   #[serde(rename = "MacAddress")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub mac_address : Option<String>
}


// A summary of the container's network settings
#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkSettings {
   #[serde(rename = "Networks")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub networks : Option<HashMap<String,EndpointSettings>>,
}

// An open port on a container
#[derive(Serialize, Deserialize, Debug)]
pub struct Port {
   #[serde(rename = "IP")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub ip : Option<String>,

   // Port on the container
   #[serde(rename = "PrivatePort")]
   pub private_port : i32,

   // Port exposed on the host
   #[serde(rename = "PublicPort")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub public_port : Option<i32>,

   #[serde(rename = "Type")]
   pub port_type : String,
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

// Map of driver specific options
#[derive(Serialize, Deserialize, Debug)]
pub struct MountVolumeOptionsDriverConfig {
   // Name of the driver to use to create the volume.
   #[serde(rename = "Name")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub name : Option<String>,

   // key/value map of driver specific options.
   #[serde(rename = "Options")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub options : Option<HashMap<String,String>>,
}

// Optional configuration for the `volume` type.
#[derive(Serialize, Deserialize, Debug)]
pub struct MountVolumeOptions {
   // Populate volume with data from the target.
   #[serde(rename = "NoCopy")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub no_copy : Option<bool>,

   // User-defined key/value metadata.
   #[serde(rename = "Labels")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub labels : Option<HashMap<String,String>>,

   #[serde(rename = "DriverConfig")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub driver_config : Option<MountVolumeOptionsDriverConfig>
}

// Optional configuration for the `tmpfs` type.
#[derive(Serialize, Deserialize, Debug)]
pub struct MountTmpfsOptions {
   // The size for the tmpfs mount in bytes.
   #[serde(rename = "SizeBytes")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub size_bytes : Option<i64>,

   // The permission mode for the tmpfs mount in an integer.
   #[serde(rename = "Mode")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub mode : Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mount {
   // Container path.
   #[serde(rename = "Target")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub target : Option<String>,

   // The mount type. Available types:  - `bind` Mounts a file or directory from the host into the container. Must exist prior to creating the container. - `volume` Creates a volume with the given name and options (or uses a pre-existing volume with the same name and options). These are **not** removed when the container is removed. - `tmpfs` Create a tmpfs with the given options. The mount source cannot be specified for tmpfs. 
   #[serde(rename = "Type")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub mount_type : Option<String>,

   // Whether the mount should be read-only.
   #[serde(rename = "ReadOnly")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub read_only : Option<bool>,

   // Optional configuration for the `bind` type.
   // TODO: could not find the BindOptions in
   // swagger

   // #[serde(rename = "BindOptions")]
   // #[serde(skip_serializing_if = "Option::is_none")]
   // pub bind_options : Option<interface{}>,

   #[serde(rename = "VolumeOptions")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub volume_options : Option<MountVolumeOptions>,

   #[serde(rename = "TmpfsOptions")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub tmpfs_options : Option<MountTmpfsOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {

   // The ID of this container
   #[serde(rename = "Id")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub id : Option<String>,

   // The names that this container has been given
   #[serde(rename = "Names")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub names : Option<Vec<String>>,

   // The name of the image used when creating this container
   #[serde(rename = "Image")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub image : Option<String>,

   // The ID of the image that this container was created from
   #[serde(rename = "ImageID")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub image_id : Option<String>,

   // Command to run when starting the container
   #[serde(rename = "Command")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub command : Option<String>,

   // When the container was created
   #[serde(rename = "Created")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub created : Option<i64>,

   // The ports exposed by this container
   #[serde(rename = "Ports")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub ports : Option<Vec<Port>>,

   // The size of files that have been created or changed by this container
   #[serde(rename = "SizeRw")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub size_rw : Option<i64>,

   // The total size of all the files in this container
   #[serde(rename = "SizeRootFs")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub size_root_fs : Option<i64>,

   // User-defined key/value metadata.
   #[serde(rename = "Labels")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub labels : Option<HashMap<String,String>>,

   // The state of this container (e.g. `Exited`)
   #[serde(rename = "State")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub state : Option<String>,

   // Additional human-readable status of this container (e.g. `Exit 0`)
   #[serde(rename = "Status")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub status : Option<String>,

   #[serde(rename = "HostConfig")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub host_config : Option<HostConfig>,

   #[serde(rename = "NetworkSettings")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub network_settings : Option<NetworkSettings>,

   #[serde(rename = "Mounts")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub mounts : Option<Vec<Mount>>
}
