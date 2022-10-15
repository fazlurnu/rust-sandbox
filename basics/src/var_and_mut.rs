fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

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

pub fn constant(){
    const ONE_HOUR_IN_SECONDS: u32 = 1 * 60 * 60;
    println!("We use const for hardcoded value such as ONE_HOUR_IN_SECONDS: {}", ONE_HOUR_IN_SECONDS);
    println!("To use const, developer should explicitly define its datatype");
}

pub fn shadowing(){
    println!("Here is shadowing");
    println!("Shadowing spares us from having to come up with different names");

    let x = 5;
    
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in this scope is {x}");
    }

    println!("The value of x in this scope is {x}");

    let spaces = "  ";
    println!("Here, my spaces type is {}", type_of(spaces));
    let spaces = spaces.len();
    println!("Here, my spaces type is {}", type_of(spaces));
    println!("Amazing!")
}