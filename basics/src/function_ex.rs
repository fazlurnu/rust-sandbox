pub fn fahrenheit_to_celcius(temp: f32) -> f32 {
    (temp - 32.0) * 5.0/9.0
}

pub fn celcius_to_fahrenheit(temp: f32) -> f32 {
    (temp * 9.0/5.0) + 32.0
}

pub fn fibb(i: i32) -> i32 {
    if (i == 1) || (i == 2){
        1
    }
    else{
        fibb(i-1) + fibb(i-2)
    }
}