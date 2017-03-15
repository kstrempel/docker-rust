pub mod schema;

use serde_json;

use super::Client;
use super::error::DockerError;
use self::schema::Network;

pub struct NetworksClient<'a> {
    client : &'a mut Client
}

impl<'a> NetworksClient<'a> {

    pub fn new(client : &'a mut Client) -> NetworksClient {
        NetworksClient{
            client : client 
        }
    }

    pub fn networks(& mut self) -> Result<Vec<Network>, DockerError> {
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
        let mut client = Client::from_env();
        let mut image_client = NetworksClient::new(&mut client);
        let networks = image_client.networks();
        assert!(networks.is_ok());
        assert!(networks.unwrap().len()>1);
    }
}
