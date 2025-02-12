use dotenv::dotenv;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn load_enviroment() {
    INIT.call_once(|| {
        dotenv().ok();
        println!("Environment loaded");
    });
}
