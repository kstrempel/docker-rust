use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "ParentId")]
    parent_id: String,
    #[serde(rename = "RepoTags")]
    repo_tags: Vec<String>,
    #[serde(rename = "RepoDigests")]
    repo_digests: Vec<String>,
    #[serde(rename = "Created")]
    created: u64,
    #[serde(rename = "Size")]
    size: u64,
    #[serde(rename = "VirtualSize")]
    virtual_size: u64,
    #[serde(rename = "SharedSize")]
    shared_size: i64,
    #[serde(rename = "Labels")]
    labels: HashMap<String, String>,
    #[serde(rename = "Containers")]
    containers: i64
}