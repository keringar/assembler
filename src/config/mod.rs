mod v1;
mod io;

pub type LatestConfig = v1::Options;

pub trait Upgrade {
    fn upgrade(&self) -> LatestConfig;
}

pub use self::io::{load_config, write_config};
pub use self::v1::Options;
