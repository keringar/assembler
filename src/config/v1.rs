use define_config;
use config::{Upgrade, LatestConfig};

define_config!{
    #[derive(Clone)]
    Options {
        version: u32 = 1,
        window_height: u32 = 800,
        window_width: u32 = 600,
        window_title: String = "Assembler".to_string(),
    }
}

impl Upgrade for Options {
    fn upgrade(&self) -> LatestConfig {
        self.clone()
    }
}
