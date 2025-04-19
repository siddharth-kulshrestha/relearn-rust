struct X {
    x: i32,
    y: i32,
}

fn print(x: &X) {
    println!("x: {}, y: {}", x.x, x.y);
}

fn increase_X(x: &mut X) {
    x.x += 1;
}

fn increase_Y(y: &mut X) {
    y.y += 1;
}

fn main() {
    println!("Hello, world!");
    let x = X { x: 10, y: 11 };
    print(&x);
    // increase_X(x);
    increase_Y(&x);
}
