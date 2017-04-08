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

// A test to perform to check that the container is healthy.
#[derive(Serialize, Deserialize, Debug)]
pub struct HealthConfig {

   // The test to perform. Possible values are:  - `{}` inherit healthcheck from image or parent image - `{\"NONE\"}` disable healthcheck - `{\"CMD\", args...}` exec arguments directly - `{\"CMD-SHELL\", command}` run command with system's default shell 
   #[serde(rename = "Test")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub test : Option<Vec<String>>,

   // The time to wait between checks in nanoseconds. 0 means inherit.
   #[serde(rename = "Interval")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub interval : Option<i32>,

   // The time to wait before considering the check to have hung. 0 means inherit.
   #[serde(rename = "Timeout")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub timeout : Option<i32>,

   // The number of consecutive failures needed to consider a container as unhealthy. 0 means inherit.
   #[serde(rename = "Retries")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub retries : Option<i32>,
}

// An object mapping mount point paths inside the container to empty objects.
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigVolumes {

    // TODO: check type of additional_properties

   // #[serde(rename = "additionalProperties")]
   // #[serde(skip_serializing_if = "Option::is_none")]
   // pub additional_properties : Option<interface{}>;
}

// Configuration for a container that is portable between hosts
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {

   // The hostname to use for the container, as a valid RFC 1123 hostname.
   #[serde(rename = "Hostname")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub hostname : Option<String>,

   // The domain name to use for the container.
   #[serde(rename = "Domainname")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub domainname : Option<String>,

   // The user that commands are run as inside the container.
   #[serde(rename = "User")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub user : Option<String>,

   // Whether to attach to `stdin`.
   #[serde(rename = "AttachStdin")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub attach_stdin : Option<bool>,

   // Whether to attach to `stdout`.
   #[serde(rename = "AttachStdout")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub attach_stdout : Option<bool>,

   // Whether to attach to `stderr`.
   #[serde(rename = "AttachStderr")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub attach_stderr : Option<bool>,

   // An object mapping ports to an empty object in the form:  `{\"<port>/<tcp|udp>\": {}}` 
   // TODO: find types for the ports

   // #[serde(rename = "ExposedPorts")]
   // #[serde(skip_serializing_if = "Option::is_none")]
   // pub exposed_ports : Option<HashMap<String,interface{}>,

   // Attach standard streams to a TTY, including `stdin` if it is not closed.
   #[serde(rename = "Tty")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub tty : Option<bool>,

   // Open `stdin`
   #[serde(rename = "OpenStdin")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub open_stdin : Option<bool>,

   // Close `stdin` after one attached client disconnects
   #[serde(rename = "StdinOnce")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub stdin_once : Option<bool>,

   // A list of environment variables to set inside the container in the form `[\"VAR=value\", ...]` 
   #[serde(rename = "Env")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub env : Option<Vec<String>>,

   #[serde(rename = "Healthcheck")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub healthcheck : Option<HealthConfig>,

   // Command is already escaped (Windows only)
   #[serde(rename = "ArgsEscaped")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub args_escaped : Option<bool>,

   // The name of the image to use when creating the container
   #[serde(rename = "Image")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub image : Option<String>,

   #[serde(rename = "Volumes")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub volumes : Option<ConfigVolumes>,

   // The working directory for commands to run in.
   #[serde(rename = "WorkingDir")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub working_dir : Option<String>,

   // Disable networking for the container.
   #[serde(rename = "NetworkDisabled")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub network_disabled : Option<bool>,

   // MAC address of the container.
   #[serde(rename = "MacAddress")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub mac_address : Option<String>,

   // `ONBUILD` metadata that were defined in the image's `Dockerfile`.
   #[serde(rename = "OnBuild")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub on_build : Option<Vec<String>>,

   // User-defined key/value metadata.
   #[serde(rename = "Labels")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub labels : Option<HashMap<String,String>>,

   // Signal to stop a container as a string or unsigned integer.
   #[serde(rename = "StopSignal")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub stop_signal : Option<String>,

   // Timeout to stop a container in seconds.
   #[serde(rename = "StopTimeout")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub stop_timeout : Option<i32>,

   // Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
   #[serde(rename = "Shell")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub shell : Option<Vec<String>>,
}

// Information about this container's graph driver.
#[derive(Serialize, Deserialize, Debug)]
pub struct GraphDriver {

   #[serde(rename = "Name")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub name : Option<String>,

   #[serde(rename = "Data")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub data : Option<HashMap<String,String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageRootFs {

   #[serde(rename = "Type")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub image_type : Option<String>,

   #[serde(rename = "Layers")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub layers : Option<Vec<String>>,

   #[serde(rename = "BaseLayer")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub base_layer : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
   #[serde(rename = "Id")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub id : Option<String>,

   #[serde(rename = "RepoTags")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub repo_tags : Option<Vec<String>>,

   #[serde(rename = "RepoDigests")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub repo_digests : Option<Vec<String>>,

   #[serde(rename = "Parent")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub parent : Option<String>,

   #[serde(rename = "Comment")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub comment : Option<String>,

   #[serde(rename = "Created")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub created : Option<i64>,

   #[serde(rename = "Container")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub container : Option<String>,

   #[serde(rename = "ContainerConfig")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub container_config : Option<Config>,

   #[serde(rename = "DockerVersion")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub docker_version : Option<String>,

   #[serde(rename = "Author")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub author : Option<String>,

   #[serde(rename = "Config")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub config : Option<Config>,

   #[serde(rename = "Architecture")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub architecture : Option<String>,

   #[serde(rename = "Os")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub os : Option<String>,

   #[serde(rename = "Size")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub size : Option<i64>,

   #[serde(rename = "VirtualSize")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub virtual_size : Option<i64>,

   #[serde(rename = "GraphDriver")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub graph_driver : Option<GraphDriver>,

   #[serde(rename = "RootFS")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub root_fs : Option<ImageRootFs>,
}
