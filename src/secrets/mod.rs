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
    /// use docker::secrets::SecretsClient;
    /// use docker::secrets::schema::SecretSpec;
    /// let client = Client::from_env();
    /// let secret = SecretSpec{
    ///                    name: Some(String::from("DockerRustMySecret")),
    ///                    labels: Option::Some(HashMap::new()),
    ///                    data: Some(String::from("VEhJUyBJUyBOT1QgQSBSRUFMIENFUlRJRklDQVRFCg=="))
    ///                 };  
    /// let secret_client = SecretsClient::new(&client);
    /// let new_secret = secret_client.create(&secret).unwrap();
    /// let secret_id = new_secret.id.unwrap_or_default();
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
        post(self.client, "secrets/create", spec)
    }

    /// Updates a existing secret
    pub fn update(&self, id: &String, spec: &SecretSpec) -> Result<SecretSpec, DockerError> {
        let url = format!("secrets/{}/update", id);
        post(self.client, url.as_str(), spec)
    }

    /// Deletes a secret
    pub fn delete(&self, id: &String) -> Result<(), DockerError> {
        let url = format!("secrets/{}", id);
        delete(self.client, url.as_str())
    }
}

