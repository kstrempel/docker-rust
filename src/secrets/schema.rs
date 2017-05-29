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
use tasks::schema::ObjectVersion;

#[derive(Serialize, Deserialize, Debug)]
pub struct Secret {

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

   #[serde(rename = "Spec")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub spec : Option<SecretSpec>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretSpec {

   // User-defined name of the secret.
   #[serde(rename = "Name")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub name : Option<String>,

   // User-defined key/value metadata.
   #[serde(rename = "Labels")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub labels : Option<HashMap<String,String>>,

   // Base64-url-safe-encoded secret data
   #[serde(rename = "Data")]
   #[serde(skip_serializing_if = "Option::is_none")]
   pub data : Option<String>
}

