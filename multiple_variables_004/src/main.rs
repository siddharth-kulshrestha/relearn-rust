fn main() {

    let variable1 = "abc";
    let num1 = 20;
    let num2 = 3; 
    let num2 = 1; // overriding variable is allowed with let keyword because it redeclares a new variable, shadowing a variable
    // num2 = 12; // This is not allowed unless we declare a variable from mut keyword (mutable)
    println!("variable1: {}, num1: {}, num2: {}, num1+num2: {}", variable1, num1, num2, num1 + num2);
}
