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
    pub fn all(&self) -> Result<Vec<Secret>, DockerError> {
        get_vector(self.client, "secrets")
    }

    pub fn create(&self, spec: &SecretSpec) -> Result<(), DockerError> {
        post(self.client, "secrets/create", spec)
    }

    pub fn delete(&self, id: &String) -> Result<(), DockerError> {
        let url = format!("secrets/{}", id);
        delete(self.client, url.as_str())
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use Client;
        use secrets::SecretsClient;
        let client = Client::from_env();
        let secret_client = SecretsClient::new(&client);
        let secrets = secret_client.all();
        assert!(secrets.is_ok());
//        assert!(secrets.unwrap().len()==0);
    }

    #[test]
    fn create_and_delete_secret() {
        use std::collections::HashMap;
        use Client;
        use secrets::schema::SecretSpec;
        use secrets::SecretsClient;
        let client = Client::from_env();
        let secret_client = SecretsClient::new(&client);

        let secret = SecretSpec{
                        name: Some(String::from("MySecret")),
                        labels: Option::Some(HashMap::new()),
                        data: Some(String::from("VEhJUyBJUyBOT1QgQSBSRUFMIENFUlRJRklDQVRFCg=="))
                     };
        let result = secret_client.create(&secret);

        let secrets = secret_client.all();
        assert!(secrets.is_ok());
        assert!(secrets.unwrap().len()==0);        
    }

    #[test]
    fn delete_secret() {
        use secrets::SecretsClient;
        use Client;
        let client = Client::from_env();
        let secret_client = SecretsClient::new(&client);

        let result = secret_client.delete(&String::from("o12uix0o96y2px62r2i8l6wpt"));
        assert!(result.is_ok());
    }        
    
}
