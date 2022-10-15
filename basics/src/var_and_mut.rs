pub fn immutable(){
    let x = 5;
    println!("Variable x with value {x} is by default immutable");
}

pub fn mutable(){
    let mut x = 5;
    println!("The init value of x is {x}, we define it as immutable by adding 'mut'");
    x = 10;
    println!("Now the value of x is {x}");
}