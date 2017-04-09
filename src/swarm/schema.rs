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

// Dispatcher configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct SpecDispatcher {
   // The delay for an agent to send a heartbeat to the dispatcher.
   #[serde(rename = "HeartbeatPeriod")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub heartbeat_period : Option<i64>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SpecCaConfigExternalCAs {
   // Protocol for communication with the external CA (currently only `cfssl` is supported).
   #[serde(rename = "Protocol")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub protocol : Option<String>,

   // URL where certificate signing requests should be sent.
   #[serde(rename = "URL")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub url : Option<String>,

   // An object with key/value pairs that are interpreted as protocol-specific options for the external CA driver.
   #[serde(rename = "Options")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub options : Option<HashMap<String,String>>,
}


// Parameters related to encryption-at-rest.
#[derive(Serialize, Deserialize, Debug)]
pub struct SpecEncryptionConfig {
   // If set, generate a key and use it to lock data stored on the managers.
   #[serde(rename = "AutoLockManagers")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub auto_lock_managers : Option<bool>,
}

// Orchestration configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct SpecOrchestration {
   // The number of historic tasks to keep per instance or node. If negative, never remove completed or failed tasks.
   #[serde(rename = "TaskHistoryRetentionLimit")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub task_history_retention_limit : Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecTaskDefaultsLogDriver {
   #[serde(rename = "Name")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub name : Option<String>,

   #[serde(rename = "Options")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub options : Option<HashMap<String,String>>,
}

// Defaults for creating tasks in this cluster.
#[derive(Serialize, Deserialize, Debug)]
pub struct SpecTaskDefaults {

   #[serde(rename = "LogDriver")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub log_driver : Option<SpecTaskDefaultsLogDriver>,
}

// CA configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct SpecCaConfig {
   // The duration node certificates are issued for.
   #[serde(rename = "NodeCertExpiry")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub node_cert_expiry : Option<i64>,

   // Configuration for forwarding signing requests to an external certificate authority.
   #[serde(rename = "ExternalCAs")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub external_c_as : Option<Vec<SpecCaConfigExternalCAs>>,
}

// Raft configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct SpecRaft {
   // The number of log entries between snapshots.
   #[serde(rename = "SnapshotInterval")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub snapshot_interval : Option<i64>,

   // The number of snapshots to keep beyond the current snapshot.
   #[serde(rename = "KeepOldSnapshots")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub keep_old_snapshots : Option<i64>,

   // The number of log entries to keep around to sync up slow followers after a snapshot is created.
   #[serde(rename = "LogEntriesForSlowFollowers")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub log_entries_for_slow_followers : Option<i64>,

   // The number of ticks that a follower will wait for a message from the leader before becoming a 
   // candidate and starting an election. `ElectionTick` must be greater than `HeartbeatTick`.  A tick currently 
   // defaults to one second, so these translate directly to seconds currently, but this is NOT guaranteed. 
   #[serde(rename = "ElectionTick")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub election_tick : Option<i32>,

   // The number of ticks between heartbeats. Every HeartbeatTick ticks, the leader will send a heartbeat to the 
   // followers.  A tick currently defaults to one second, so these translate directly to seconds currently, 
   //but this is NOT guaranteed. 
   #[serde(rename = "HeartbeatTick")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub heartbeat_tick : Option<i32>,
}

// User modifiable swarm configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
   // Name of the swarm.
   #[serde(rename = "Name")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub name : Option<String>,

   // User-defined key/value metadata.
   #[serde(rename = "Labels")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub labels : Option<HashMap<String,String>>,

   #[serde(rename = "Orchestration")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub orchestration : Option<SpecOrchestration>,

   #[serde(rename = "Raft")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub raft : Option<SpecRaft>,

   #[serde(rename = "Dispatcher")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub dispatcher : Option<SpecDispatcher>,

   #[serde(rename = "CAConfig")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub ca_config : Option<SpecCaConfig>,

   #[serde(rename = "EncryptionConfig")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub encryption_config : Option<SpecEncryptionConfig>,

   #[serde(rename = "TaskDefaults")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub task_defaults : Option<SpecTaskDefaults>,
}

// User modifiable swarm configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct Swarm {
   #[serde(rename = "ID")]
   pub id : String,
    
   #[serde(rename = "Spec")]
   pub spec : Spec,

   #[serde(rename = "TaskDefaults")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub task_defaults : Option<SpecTaskDefaults>,

   #[serde(rename = "EncryptionConfig")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub encryption_config : Option<SpecEncryptionConfig>,

   #[serde(rename = "JoinTokens")]
   pub join_tokens : HashMap<String,String>,
   
   #[serde(rename = "Dispatcher")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub dispatcher : Option<SpecDispatcher>,
}