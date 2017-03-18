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
pub struct Image {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "ParentId")]
    pub parent_id: String,
    #[serde(rename = "RepoTags")]
    pub repo_tags: Vec<String>,
    #[serde(rename = "RepoDigests")]
    pub repo_digests: Vec<String>,
    #[serde(rename = "Created")]
    pub created: u64,
    #[serde(rename = "Size")]
    pub size: u64,
    #[serde(rename = "VirtualSize")]
    pub virtual_size: u64,
    #[serde(rename = "SharedSize")]
    pub shared_size: i64,
    #[serde(rename = "Labels")]
    pub labels: HashMap<String, String>,
    #[serde(rename = "Containers")]
    pub containers: i64
}