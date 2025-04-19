
fn let_with_mutable_types() {
    // let x;
    // println!("{}", x);
    /*
    error[E0282]: type annotations needed
 --> src/main.rs:3:9
  |
3 |     let x;
  |         ^
  |
help: consider giving `x` an explicit type
  |
3 |     let x: /* Type */;
  |          ++++++++++++

For more information about this error, try `rustc --explain E0282`.
    */

    // let mut y;
    // println!("{}", y);
    /*
    error[E0282]: type annotations needed
  --> src/main.rs:20:9
   |
20 |     let mut y;
   |         ^^^^^
   |
help: consider giving `y` an explicit type
   |
20 |     let mut y: /* Type */;
   |              ++++++++++++

For more information about this error, try `rustc --explain E0282`.
     */

    let x;
    x = 5; // One time assignment of immutable variable is allowed.

    println!("{}", x);

    let mut y; 
    y = 10; 
    y += 5;
    println!("{}", y);


}

fn main() {
    println!("Hello, world!");
    let_with_mutable_types();
}

/*
The rule is: if you have an immutable variable, you can only assign to it once. Usually that will be at the same time as you use let, but it can be later. If you have a mutable variable, you can assign as many times as you want.
*/