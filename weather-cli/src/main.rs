fn main() {
    println!("Hello, world!");
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
    let api_token = std::env::var("API_TOKEN")
        .expect("expected there to be an api token");
    dbg!(api_token);
}
