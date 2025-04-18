fn say_hello(name: &str) {
    println!("Hello, world! Welcome {} to the world of rust!", name);
}

fn main() {
    let name = "Sid";
    say_hello(name);
}
