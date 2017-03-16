pub mod schema;

use serde_json;

use std::cell::RefCell;

use super::Client;
use super::error::DockerError;
use self::schema::Network;

pub struct NetworksClient<'a> {
    client : &'a Client
}

impl<'a> NetworksClient<'a> {

    pub fn new(client : &Client) -> NetworksClient {
        NetworksClient {
            client : client 
        }
    }

    pub fn all(&self) -> Result<Vec<Network>, DockerError> {
        let result = self.client.get("networks").unwrap();
        let networks : Vec<Network> = serde_json::from_str(result.as_str()).unwrap();
        Ok(networks)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use Client;
        use networks::NetworksClient;
        let client = Client::from_env();
        let image_client = NetworksClient::new(&client);
        let networks = image_client.all();
        assert!(networks.is_ok());
        assert!(networks.unwrap().len()>1);
    }
}
