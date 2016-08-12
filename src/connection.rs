use std::net::IpAddr;

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection {
    url: Option<String>, // inner type depends on how mokou will accept foreign connections
}

impl Connection {
    pub fn new() -> Connection {
        Connection { url: None }
    }

    pub fn connect(&mut self, addr: IpAddr) -> Result<(), &'static str> {
        self.url = Some(format!("{}", addr));
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionRequest {
    pub url: String,
}
