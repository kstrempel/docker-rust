pub mod schema;

use super::common::get;
use super::Client;
use super::error::DockerError;

use self::schema::Image;


pub struct ImagesClient<'a> {
    client : &'a Client
}

impl<'a> ImagesClient<'a> {

    pub fn new(client : &Client) -> ImagesClient {
        ImagesClient {
            client : client 
        }
    }

    pub fn all(&self) -> Result<Vec<Image>, DockerError> {
        get(self.client, "images/json")
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use Client;
        use images::ImagesClient;
        let client = Client::from_env();
        let image_client = ImagesClient::new(&client);
        let images = image_client.all();
        assert!(images.is_ok());
        assert!(images.unwrap().len()>1);
    }
}
