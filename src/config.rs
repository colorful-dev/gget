pub struct Config {
    pub default_save_path: String,
    pub default_use_proxy: bool,
}

impl Config {
    pub fn load() -> Self {
        // read config from file ~/.gget.yaml
        todo!();
    }
}
