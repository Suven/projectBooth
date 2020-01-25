use std::env;

pub fn get(key: &str) -> String {
    match env::var(key) {
        Err(_e) => panic!("Undefined config `{}`", key),
        Ok(val) => val,
    }
}

pub fn get_u8(key: &str) -> u8 {
    let v = get(key);
    match v.parse::<u8>() {
        Err(_e) => panic!("config `{}` was expected to be u8, but was `{}`"),
        Ok(v) => v,
    }
}
