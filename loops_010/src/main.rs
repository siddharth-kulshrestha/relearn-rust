fn while_loop_on_each_numerical_combination(low: i32, high: i32) {
    let mut i = low;
    while i < high {
      println!("i: {}", i);
      i += 1;
      let mut j = high; 
      while j > low {
        println!("\tj: {}", j);
        j -= 1; 
      }
    }
}

fn for_loop_on_each_number(low: i32, high: i32) {

    for i in low..high {
        println!("{}", i);
    }
}

fn loop_statement_example(low: i32, high: i32) {
    let mut i = low;
    loop {
        println!("\t\t{}", i);
        i += 1;
        if i == high {
            break;
        }
    }
}

fn main() {
    println!("Hello, world!");
    while_loop_on_each_numerical_combination(12, 21);
    for_loop_on_each_number(10, 20);
    loop_statement_example(50, 100);
}
