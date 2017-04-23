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

use images::schema::HealthConfig;
use volumes::schema::Mount;

// Define resources reservation.
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSpecResourcesReservation {

   // CPU reservation in units of 10<sup>-9</sup> CPU shares.
   #[serde(rename = "NanoCPUs")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub nano_cp_us : Option<i64>,

   // Memory reservation in Bytes.
   #[serde(rename = "MemoryBytes")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub memory_bytes : Option<i64>
}

// Define resources limits.
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSpecResourcesLimits {

   // CPU limit in units of 10<sup>-9</sup> CPU shares.
   #[serde(rename = "NanoCPUs")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub nano_cp_us : Option<i64>,

   // Memory limit in Bytes.
   #[serde(rename = "MemoryBytes")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub memory_bytes : Option<i64>
}


// Specification for DNS related configurations in resolver configuration file (`resolv.conf`).
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSpecContainerSpecDnsConfig {

   // The IP addresses of the name servers.
   #[serde(rename = "Nameservers")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub nameservers : Option<Vec<String>>,

   // A search list for host-name lookup.
   #[serde(rename = "Search")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub search : Option<Vec<String>>,

   // A list of internal resolver variables to be modified (e.g., `debug`, `ndots:3`, etc.).
   #[serde(rename = "Options")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub options : Option<Vec<String>>
}


// File represents a specific target that is backed by a file.
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSpecContainerSpecFile {

   // Name represents the final filename in the filesystem.
   #[serde(rename = "Name")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub name : Option<String>,

   // UID represents the file UID.
   #[serde(rename = "UID")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub uid : Option<String>,

   // GID represents the file GID.
   #[serde(rename = "GID")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub gid : Option<String>,

   // Mode represents the FileMode of the file.
   #[serde(rename = "Mode")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub mode : Option<i32>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSpecContainerSpecSecrets {

   #[serde(rename = "File")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub file : Option<TaskSpecContainerSpecFile>,

   // SecretID represents the ID of the specific secret that we're referencing.
   #[serde(rename = "SecretID")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub secret_id : Option<String>,

   // SecretName is the name of the secret that this references, but this is just provided for lookup/display purposes. The secret in the reference will be identified by its ID. 
   #[serde(rename = "SecretName")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub secret_name : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSpecResources {

   #[serde(rename = "Limits")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub limits : Option<TaskSpecResourcesLimits>,

   #[serde(rename = "Reservation")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub reservation : Option<TaskSpecResourcesReservation>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSpecPlacement {

   // An array of constraints.
   #[serde(rename = "Constraints")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub constraints : Option<Vec<String>>
}

// Specification for the restart policy which applies to containers created as part of this service.
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSpecRestartPolicy {  

   // Condition for restart.
   #[serde(rename = "Condition")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub condition : Option<String>,

   // Delay between restart attempts.
   #[serde(rename = "Delay")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub delay : Option<i64>,

   // Maximum attempts to restart a given container before giving up (default value is 0, which is ignored).
   #[serde(rename = "MaxAttempts")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub max_attempts : Option<i64>,

   // Windows is the time window used to evaluate the restart policy (default value is 0, which is unbounded).
   #[serde(rename = "Window")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub window : Option<i64>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSpecLogDriver {

   #[serde(rename = "Name")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub name : Option<String>,

   #[serde(rename = "Options")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub options : Option<HashMap<String,String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSpecContainerSpec {

   // The image name to use for the container.
   #[serde(rename = "Image")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub image : Option<String>,

   // User-defined key/value data.
   #[serde(rename = "Labels")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub labels : Option<HashMap<String,String>>,

   // The command to be run in the image.
   #[serde(rename = "Command")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub command : Option<Vec<String>>,

   // Arguments to the command.
   #[serde(rename = "Args")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub args : Option<Vec<String>>,

   // The hostname to use for the container, as a valid RFC 1123 hostname.
   #[serde(rename = "Hostname")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub hostname : Option<String>,

   // A list of environment variables in the form `VAR=value`.
   #[serde(rename = "Env")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub env : Option<Vec<String>>,

   // The working directory for commands to run in.
   #[serde(rename = "Dir")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub dir : Option<String>,

   // The user inside the container.
   #[serde(rename = "User")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub user : Option<String>,

   // A list of additional groups that the container process will run as.
   #[serde(rename = "Groups")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub groups : Option<Vec<String>>,

   // Whether a pseudo-TTY should be allocated.
   #[serde(rename = "TTY")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub tty : Option<bool>,

   // Open `stdin`
   #[serde(rename = "OpenStdin")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub open_stdin : Option<bool>,

   // Mount the container's root filesystem as read only.
   #[serde(rename = "ReadOnly")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub read_only : Option<bool>,

   // Specification for mounts to be added to containers created as part of the service.
   #[serde(rename = "Mounts")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub mounts : Option<Vec<Mount>>,

   // Amount of time to wait for the container to terminate before forcefully killing it.
   #[serde(rename = "StopGracePeriod")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub stop_grace_period : Option<i64>,

   #[serde(rename = "HealthCheck")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub health_check : Option<HealthConfig>,

   // A list of hostnames/IP mappings to add to the container's `/etc/hosts` file. The format of extra hosts on swarmkit is specified in: http://man7.org/linux/man-pages/man5/hosts.5.html   IP_address canonical_hostname [aliases...] 
   #[serde(rename = "Hosts")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub hosts : Option<Vec<String>>,

   #[serde(rename = "DNSConfig")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub dns_config : Option<TaskSpecContainerSpecDnsConfig>,

   // Secrets contains references to zero or more secrets that will be exposed to the service.
   #[serde(rename = "Secrets")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub secrets : Option<Vec<TaskSpecContainerSpecSecrets>>
}


// TODO: that looks weird.. 
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskState {
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSpecNetworks {

   #[serde(rename = "Target")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub target : Option<String>,

   #[serde(rename = "Aliases")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub aliases : Option<Vec<String>>
}

// User modifiable task configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSpec {

   #[serde(rename = "ContainerSpec")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub container_spec : Option<TaskSpecContainerSpec>,

   #[serde(rename = "Resources")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub resources : Option<TaskSpecResources>,

   #[serde(rename = "RestartPolicy")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub restart_policy : Option<TaskSpecRestartPolicy>,

   #[serde(rename = "Placement")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub placement : Option<TaskSpecPlacement>,

   // A counter that triggers an update even if no relevant parameters have been changed.
   #[serde(rename = "ForceUpdate")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub force_update : Option<i32>,

   #[serde(rename = "Networks")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub networks : Option<Vec<TaskSpecNetworks>>,

   #[serde(rename = "LogDriver")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub log_driver : Option<TaskSpecLogDriver>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStatusContainerStatus {

   #[serde(rename = "ContainerID")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub container_id : Option<String>,

   #[serde(rename = "PID")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub pid : Option<i32>,

   #[serde(rename = "ExitCode")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub exit_code : Option<i32>
}


// The version number of the object such as node, service, etc. This is needed to avoid conflicting writes. The client
// must send the version number along with the modified specification when updating these objects. This approach ensures 
// safe concurrency and determinism in that the change on the object may not be applied if the version number has changed 
// from the last read. In other words, if two update requests specify the same base version, only one of the requests can 
// succeed. As a result, two separate update requests that happen at the same time will not unintentially overwrite each other. 
#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectVersion {

   #[serde(rename = "Index")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub index : Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStatus {

   #[serde(rename = "Timestamp")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub timestamp : Option<String>,

   #[serde(rename = "State")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub state : Option<TaskState>,

   #[serde(rename = "Message")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub message : Option<String>,

   #[serde(rename = "Err")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub err : Option<String>,

   #[serde(rename = "ContainerStatus")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub container_status : Option<TaskStatusContainerStatus>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Task {

   // The ID of the task.
   #[serde(rename = "ID")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub id : Option<String>,

   #[serde(rename = "Version")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub version : Option<ObjectVersion>,

   #[serde(rename = "CreatedAt")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub created_at : Option<String>,

   #[serde(rename = "UpdatedAt")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub updated_at : Option<String>,

   // Name of the task.
   #[serde(rename = "Name")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub name : Option<String>,

   // User-defined key/value metadata.
   #[serde(rename = "Labels")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub labels : Option<HashMap<String, String>>,

   #[serde(rename = "Spec")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub spec : Option<TaskSpec>,

   // The ID of the service this task is part of.
   #[serde(rename = "ServiceID")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub service_id : Option<String>,

   #[serde(rename = "Slot")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub slot : Option<i32>,

   // The ID of the node that this task is on.
   #[serde(rename = "NodeID")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub node_id : Option<String>,

   #[serde(rename = "Status")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub status : Option<TaskStatus>,

   #[serde(rename = "DesiredState")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub desired_state : Option<TaskState>
}
