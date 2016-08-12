#[derive(Serialize, Deserialize, Debug)]
pub struct Agent {
    name: String,
    active: bool,
    details: String,
}

impl Agent {
    pub fn new(name: &str, active: bool) -> Agent {
        Agent {
            name: name.to_owned(),
            active: active,
            details: format!("Super interesting details for {}", name),
        }
    }
}
