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

use self::schema::Swarm;

endpoint!(SwarmClient);

impl<'a> SwarmClient<'a> {
    pub fn get(&self) -> Result<Swarm, DockerError> {
        get(self.client, "swarm")
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use Client;
        use swarm::SwarmClient;
        let client = Client::from_env();
        let swarm_client = SwarmClient::new(&client);
        let swarm = swarm_client.get();
        assert!(swarm.is_ok());
    }
}
