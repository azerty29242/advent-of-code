use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let session_cookie = env::var("SESSION")
        .expect("Please specify the session cookie");
    println!("{}", session_cookie);
}
