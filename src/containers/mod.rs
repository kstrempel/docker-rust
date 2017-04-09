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

use self::schema::Container;

endpoint!(ContainersClient);

impl<'a> ContainersClient<'a> {

    pub fn all(&self) -> Result<Vec<Container>, DockerError> {
        get_vector(self.client, "containers/json")
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use Client;
        use containers::ContainersClient;
        let client = Client::from_env();
        let container_client = ContainersClient::new(&client);
        let containers = container_client.all();
        assert!(containers.is_ok());
        assert!(containers.unwrap().len()>1);
    }
}
