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
   pub mode : Option<i32>
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
   pub options : Option<HashMap<String,String>>
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
   //#[serde(rename = "BindOptions")]
   //#[serde(skip_serializing_if = "Option::is_none")]
   //pub bind_options : Option<interface{}>,

   #[serde(rename = "VolumeOptions")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub volume_options : Option<MountVolumeOptions>,

   #[serde(rename = "TmpfsOptions")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub tmpfs_options : Option<MountTmpfsOptions>
}

