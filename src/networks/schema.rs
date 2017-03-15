use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct IPAM {
    #[serde(rename = "Driver")]
    driver: String,
    #[serde(rename = "Options")]
    options: String,
    #[serde(rename = "Config")]
    config: Vec<HashMap<String,String>>,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Created")]
    created: String,
    #[serde(rename = "Scope")]
    scope: String,
    #[serde(rename = "Driver")]
    driver: String,
    #[serde(rename = "EnableIPv6")]
    enable_ip_v6: bool,
    #[serde(rename = "Internal")]
    internal: bool,
    #[serde(rename = "Attachable")]
    attachable: bool,
    #[serde(rename = "IPAM")]
    ipam : IPAM,
    #[serde(rename = "Containers")]
    containers : HashMap<String, HashMap<String,String>>,
    #[serde(rename = "Options")]
    options : HashMap<String,String>,
    #[serde(rename = "Labels")]
    labels: HashMap<String, String>,    
}