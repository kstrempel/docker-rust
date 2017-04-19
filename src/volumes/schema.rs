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
pub struct VolumeUsageData {

   // The disk space used by the volume (local driver only)
   #[serde(rename = "Size")]
   pub size : i32,

   // The number of containers referencing this volume.
   #[serde(rename = "RefCount")]
   pub ref_count : i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Volume {

   // Name of the volume.
   #[serde(rename = "Name")]
   pub name : String,

   // Name of the volume driver used by the volume.
   #[serde(rename = "Driver")]
   pub driver : String,

   // Mount path of the volume on the host.
   #[serde(rename = "Mountpoint")]
   pub mountpoint : String,

   // Low-level details about the volume, provided by the volume driver. Details are returned as a map with key/value pairs: `{\"key\":\"value\",\"key2\":\"value2\"}`.  The `Status` field is optional, and is omitted if the volume driver does not support this feature. 
   #[serde(rename = "Status")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub status : Option<HashMap<String,String>>,

   // User-defined key/value metadata.
   #[serde(rename = "Labels")]
   pub labels : HashMap<String,String>,

   // The level at which the volume exists. Either `global` for cluster-wide, or `local` for machine level.
   #[serde(rename = "Scope")]
   pub scope : String,

   // The driver specific options used when creating the volume.
   #[serde(rename = "Options")]
   pub options : HashMap<String,String>,

   #[serde(rename = "UsageData")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub usage_data : Option<VolumeUsageData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Volumes {
    #[serde(rename = "Volumes")]
    pub volumes : Vec<Volume>,

    #[serde(rename = "Warnings")]
    pub warnings : Vec<String>
}
