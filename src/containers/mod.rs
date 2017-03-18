pub mod schema;

use super::common::*;
use super::Client;
use super::error::DockerError;

use self::schema::Container;

pub struct ContainersClient<'a> {
    client : &'a Client
}

impl<'a> ContainersClient<'a> {

    pub fn new(client : &Client) -> ContainersClient {
        ContainersClient {
            client : client 
        }
    }

    pub fn all(&self) -> Result<Vec<Container>, DockerError> {
        get(self.client, "containers/json")
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
