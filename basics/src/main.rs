// mod var_and_mut;
// mod data_types;
mod function_ex;

fn main() {
    // var_and_mut::immutable();
    // var_and_mut::mutable();
    // var_and_mut::constant();
    // var_and_mut::shadowing();

    // data_types::scalar_types();
    // data_types::compound_types();

    // -----function sections-------
    let index = 10;
    println!("The {index}th fibbonacci is: {}", function_ex::fibb(index));

    let temp_f = 32.0;
    let temp_c = 100.0;

    println!("{} deg fahrenheit is {} deg celcius", temp_f, function_ex::fahrenheit_to_celcius(temp_f));
    println!("{} deg celcius is {} deg fahrenheit", temp_c, function_ex::celcius_to_fahrenheit(temp_c));
}
