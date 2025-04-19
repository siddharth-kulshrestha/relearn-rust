fn print_x(x: i32) {
    println!("{}", x);
}

fn return_x_sum_1(x: i32) -> i32 {
    x + 1
}

fn print_y(y: &str) {
    println!("{}", y);
}

fn print_y_with_hello(y: &str) {
   println!("{} Hello", y); 
}

fn main() {
    println!("Hello, world!");
    let x = 1;
    // return_x_sum_1(x);
    print_x(x);
    return_x_sum_1(x);
    /*
        Everything works here because we are using x as i32 which already implements the Copy Trait.
    */

    let y = "ABCD";
    print_y(y);
    print_y_with_hello(y);
}
