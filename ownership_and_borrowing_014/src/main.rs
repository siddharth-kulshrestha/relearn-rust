struct X {
    x: i32,
    y: i32,
}

fn print_x_X(x: X) -> X {
    println!("{}", x.x);
    x
}

fn print_x_X_using_borrowing(x: &X) {
    println!("{}", x.x);
}

fn print_y_X(x: X) {
    println!("{}", x.y);
}

fn my_print(x: &i32) {
    println!("{}", x);
}

fn increase(x: &mut i32) {
    x += 1;
    println!("{}", x);
}

fn main() {
    println!("Hello, world!");
    let x = X{
        x: 12,
        y: 15,
    };
    // let x = print_x_X(x);
    print_x_X_using_borrowing(&x);
    print_y_X(x);

    let o = 60;
    my_print(&o);
    increase(&o);


}
