fn fact(num: u64) -> u64 {
    if num == 0 {
        1
    } else {
        num * fact(num-1)
    }
}

fn fact1(num: u64) -> u64 {
    if num == 0 {
        return 1;
    }
    return num * fact(num-1)
} // valid

// fn fact2(num: u64) -> u64 {
//     if num == 0 {
//         1
//     }
//     num * fact(num-1)
// } // invalid

fn main() {
    println!("Factorial of x: 5 is {}", fact(5));
}
