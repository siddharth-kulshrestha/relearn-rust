fn double(x: i32) -> i32 {
   let res: i32 = x * 2;
   return res;
}

fn add(x: i32, z: i32) -> i32 {
    x+z
}

fn main() -> () {
    let y: i32 = 17;
    let z = double(y);

    println!("The double of number {} is {}", y, z);
    println!("The sum of number {} & {} is {}", y, 63, add(y, 63));
}
