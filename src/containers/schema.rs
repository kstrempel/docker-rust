use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Port {
    #[serde(rename = "PrivatePort")]
    private_port: u64,
    #[serde(rename = "PublicPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    public_port: Option<u64>,
    #[serde(rename = "Type")]
    port_type: String
}
    
#[derive(Serialize, Deserialize, Debug)]
pub struct Bridge {
    #[serde(rename = "NetworkID")]
    network_id : String,
    #[serde(rename = "EndpointID")]
    endpoint_id : String,
    #[serde(rename = "Gateway")]
    gateway : String,
    #[serde(rename = "IPAddress")]
    ip_address : String,
    #[serde(rename = "IPPrefixLen")]
    ip_prefix_len: u64,
    #[serde(rename = "IPv6Gateway")]
    ipv6_gateway : String,
    #[serde(rename = "GlobalIPv6Address")]
    global_ipv6_address : String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    global_ipv6_preflix_len:  u64,
    #[serde(rename = "MacAddress")]
    mac_address : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mount {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "Source")]
    source : String,
    #[serde(rename = "Destination")]
    destination: String,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    driver: Option<String>,
    #[serde(rename = "Mode")]
    mode : String, 
    #[serde(rename = "RW")]
    rw : bool,
    #[serde(rename = "Propagation")]
    propagation : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Names")]
    names: Vec<String>,
    #[serde(rename = "Image")]
    image: String,
    #[serde(rename = "ImageID")]
    image_id: String,
    #[serde(rename = "Command")]
    command: String,
    #[serde(rename = "Created")]
    created: u64,
    #[serde(rename = "State")]
    state: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "Ports")]
    ports: Vec<Port>,
    #[serde(rename = "Labels")]
    labels: HashMap<String, String>,
    #[serde(rename = "SizeRw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    size_rw: Option<u64>,
    #[serde(rename = "SizeRootFs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    size_root_fs: Option<u64>,
    #[serde(rename = "HostConfig")]
    host_config: HashMap<String,String>,
    #[serde(rename = "NetworkSettings")]
    network_settings: HashMap<String,HashMap<String,Bridge>>,
    #[serde(rename = "Mounts")]
    mounts: Vec<Mount>
}