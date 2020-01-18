use dotenv::dotenv;

pub fn load_config() {
    dotenv().ok();
}
