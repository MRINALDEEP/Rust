fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = 7;
    let z = x + y;

    println!("z is {}",z);
    next_birthday("Jake", 33);
    next_birthday("Vibhuti", 56);
    println!("The Answer is {}",square(3))
}

fn next_birthday(name: &str,current_age: u8){
    let next_age = current_age + 1;
    println!("Hi {}, on your next birthday, you will be {}!",name, next_age);
}
fn square(num: i64) -> i64 {
    num*num
}


