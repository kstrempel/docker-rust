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

pub mod schema;

use super::common::*;
use super::Client;
use super::error::DockerError;

use self::schema::{Secret, SecretSpec};

endpoint!(SecretsClient);

impl<'a> SecretsClient<'a> {
    
    /// Returns a list of all secrets
    pub fn all(&self) -> Result<Vec<Secret>, DockerError> {
        get_vector(self.client, "secrets")
    }

    /// Returns a specific secret
    ///
    /// # Example
    /// ```    
    /// use std::collections::HashMap;
    /// use docker::Client;
    /// use docker::secrets::schema::SecretSpec;
    /// let client = Client::from_env();
    /// let secret_client = client.secrets();
    /// let labels : HashMap<String,String> = [(String::from("label1"),String::from("entry1")), 
    ///                                        (String::from("label2"),String::from("entry2"))].iter().cloned().collect();
    /// let secret = SecretSpec{
    ///                    name: Some(String::from("DockerRustMySecret")),
    ///                    labels: Option::Some(labels),
    ///                    data: Some(String::from("VEhJUyBJUyBOT1QgQSBSRUFMIENFUlRJRklDQVRFCg=="))
    ///                 };  
    /// let new_secret = secret_client.create(&secret).unwrap();
    /// let secret_id = new_secret.id.unwrap();
    /// let inspected_secret = secret_client.inspect(&secret_id).unwrap();
    /// let inspected_secret_spec = inspected_secret.spec.unwrap();
    /// assert!(inspected_secret_spec.name == secret.name);  
    /// secret_client.delete(&secret_id).unwrap();
    /// ```    
    pub fn inspect(&self, id: &String) -> Result<Secret, DockerError> {
        let url = format!("secrets/{}", id);
        get(self.client, url.as_str())
    }

    /// Creates a new secret
    pub fn create(&self, spec: &SecretSpec) -> Result<Secret, DockerError> {
        post(self.client, "secrets/create", spec, 201)
    }

    /// Updates a secret
    ///
    /// # Example
    /// ```    
    /// use std::collections::HashMap;
    /// use docker::Client;
    /// use docker::secrets::schema::SecretSpec;
    /// let client = Client::from_env();
    /// let secret_client = client.secrets();
    /// let secret = SecretSpec{
    ///                    name: Some(String::from("UpdateDockerRustSecret")),
    ///                    labels: Option::Some(HashMap::new()),
    ///                    data: Some(String::from("VEhJUyBJUyBOT1QgQSBSRUFMIENFUlRJRklDQVRFCg=="))
    ///                 };  
    /// let new_secret = secret_client.create(&secret).unwrap();
    /// let secret_id = new_secret.id.unwrap_or_default();
    /// let to_update_secret = secret_client.inspect(&secret_id).unwrap();
    /// let version = to_update_secret.version.unwrap().index.unwrap();
    /// let mut to_update_spec = to_update_secret.spec.unwrap();
    /// let labels : HashMap<String,String> = [(String::from("label1"),String::from("entry1")), 
    ///                                        (String::from("label2"),String::from("entry2"))].iter().cloned().collect();
    /// to_update_spec.labels = Option::Some(labels);
    /// assert!(secret_client.update(&secret_id, &to_update_spec, version).is_ok());
    /// secret_client.delete(&secret_id).unwrap();
    /// ```    
    pub fn update(&self, id: &String, spec: &SecretSpec, version: i64) -> Result<(), DockerError> {
        let url = format!("secrets/{}/update?version={}", id, version);
        update(self.client, url.as_str(), spec, 200)
    }

    /// Deletes a secret
    pub fn delete(&self, id: &String) -> Result<(), DockerError> {
        let url = format!("secrets/{}", id);
        delete(self.client, url.as_str())
    }
}

