pub mod schema;

use serde_json;

use super::Client;
use super::error::DockerError;
use self::schema::Image;

pub struct ImagesClient<'a> {
    client : &'a mut Client
}

impl<'a> ImagesClient<'a> {

    pub fn new(client : &'a mut Client) -> ImagesClient {
        ImagesClient{
            client : client 
        }
    }

    pub fn images(& mut self) -> Result<Vec<Image>, DockerError> {
        let result = self.client.get("images/json").unwrap();
        let images : Vec<Image> = serde_json::from_str(result.as_str()).unwrap();
        Ok(images)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use Client;
        use images::ImagesClient;
        let mut client = Client::from_env();
        let mut image_client = ImagesClient::new(&mut client);
        let images = image_client.images();
        assert!(images.is_ok());
        assert!(images.unwrap().len()>1);
    }
}
