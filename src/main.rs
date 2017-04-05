#[macro_use]
extern crate define_config;

mod config;

fn main() {
    let mut config = config::load_config();
    println!("{:?}", config);

    config.window_title.push_str("a");

    println!("{:?}", config);
    config::write_config(&config);

    std::thread::sleep(std::time::Duration::new(5, 0));
}
