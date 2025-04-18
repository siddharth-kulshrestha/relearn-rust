

fn main() {
    println!("Hello, world!");

    let time = 20;
    let temp = 30; 

    let greeting = if time > 12 && time < 18 {
        "Good afternoon"
    } else if time <= 12 {
        "Good morning"
    } else {
        "Good night"
    };

    println!("{}", greeting);

    println!("{}", if temp <= 10 {
        "It's cold!"
    } else if temp < 25 {
        "It's nice"
    } else if temp <= 40 {
        "It's warm"
    } else {
        "It's hot"
    });
}
