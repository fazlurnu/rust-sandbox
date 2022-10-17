fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn scalar_types(){
    let x : u16 = 10; // u16
    let y = 2.0; // f64
    let z : f32 = 3.0; // f32

    println!("value of x is {x}, with data type {}", type_of(x));
    println!("value of y is {y}, with data type {}", type_of(y));
    println!("value of z is {z}, with data type {}", type_of(z));

    // Notes on numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // Boolean
    let t = true;
    let f : bool = false;

    println!("t is {t} and f is {f}, both are {}", type_of(t));

    // Char
    let c = 'z';
    let w : char = 'ℤ'; // with explicit type annotation
    println!("c and w are both {}", type_of(c));
}

pub fn compound_types(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("\n");
    println!("Here is tup with {}, {}, and {}", x, y, z);
    println!("Tuple can have different data type and it is a fixed size var!");
    println!("We can also access tuple using dot like this {}, {}, and {}", tup.0, tup.1, tup.2);

    println!("\n");
    let a = [1, 2, 3, 4, 5];
    println!("Here is 'a', an array, should contain same data type.");
    
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Here is 'b', another way of declaring an array");

    let c = [3; 5];
    println!("Also this 'c' which will be {}, {}, {}, {}, and {}", c[0], c[1], c[2], c[3], c[4]);
}