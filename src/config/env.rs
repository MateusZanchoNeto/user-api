use dotenv::dotenv;

pub fn load_enviroment() {
    dotenv().ok();
}
