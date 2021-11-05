
pub struct TCP_Client {
    hostname : str;
    port : u32;
    settings : Box<Settings>;
}

impl Client for TCP_Client  {
    pub fn new(&self, settings_file: &'static str) -> Self {
        self.settings = Box::new(Settings(settings_file));
    }
}
